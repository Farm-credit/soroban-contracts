#![no_std]

mod error;
mod storage;

#[cfg(test)]
mod test;

pub use error::Error;
use soroban_sdk::{contract, contractimpl, Address, Env, String, Symbol, Vec};

use crate::storage::{RoleType, Proposal, ProposalAction};

fn execute_proposal_logic(env: &Env, action: &ProposalAction) -> Result<(), Error> {
    match action {
        ProposalAction::ChangeSuperAdmins { admins, threshold } => {
            if admins.is_empty() {
                return Err(Error::EmptyAdmins);
            }
            if *threshold == 0 || *threshold > admins.len() {
                return Err(Error::InvalidThreshold);
            }

            // Revoke admin status for any removed super admins
            let old_admins = storage::read_super_admins(env);
            for old_admin in old_admins.iter() {
                let mut is_still_admin = false;
                for new_admin in admins.iter() {
                    if new_admin == old_admin {
                        is_still_admin = true;
                        break;
                    }
                }
                if !is_still_admin {
                    if storage::is_super_admin(env, &old_admin) {
                        storage::revoke_admin(env, &old_admin);
                        storage::remove_role(env, &old_admin);
                    }
                }
            }

            // Write new list and threshold
            storage::write_super_admins(env, admins);
            storage::write_super_admin_threshold(env, *threshold);

            // Assign new super admin roles
            for admin in admins.iter() {
                storage::write_role(env, &admin, RoleType::SuperAdmin);
                storage::write_admin(env, &admin);
            }
        }
        ProposalAction::SetTimelockDelay(delay) => {
            storage::write_timelock_delay(env, *delay);
        }
        ProposalAction::GrantAdmin(account) => {
            storage::write_admin(env, account);
            storage::write_role(env, account, RoleType::Admin);
        }
        ProposalAction::RevokeAdmin(account) => {
            if storage::is_super_admin(env, account) {
                return Err(Error::CannotRemoveSuperAdmin);
            }
            storage::revoke_admin(env, account);
            storage::remove_role(env, account);
        }
        ProposalAction::AssignRolesBatch(assignments) => {
            for assignment in assignments.iter() {
                let (account, role_str) = assignment;
                let role_type = match role_str {
                    r if r == Symbol::new(env, "Admin") => RoleType::Admin,
                    r if r == Symbol::new(env, "Verifier") => RoleType::Verifier,
                    r if r == Symbol::new(env, "Trader") => RoleType::Trader,
                    _ => return Err(Error::InvalidRole),
                };
                storage::write_role(env, &account, role_type);
                if role_type == RoleType::Admin {
                    storage::write_admin(env, &account);
                }
            }
        }
        ProposalAction::RevokeRolesBatch(revocations) => {
            for revocation in revocations.iter() {
                let (account, role_str) = revocation;
                if storage::is_super_admin(env, &account) {
                    return Err(Error::CannotRemoveSuperAdmin);
                }
                let expected_role = match role_str {
                    r if r == Symbol::new(env, "Admin") => RoleType::Admin,
                    r if r == Symbol::new(env, "Verifier") => RoleType::Verifier,
                    r if r == Symbol::new(env, "Trader") => RoleType::Trader,
                    _ => return Err(Error::InvalidRole),
                };
                if let Some(current_role) = storage::read_role(env, &account) {
                    if current_role == expected_role {
                        match current_role {
                            RoleType::Admin => {
                                storage::revoke_admin(env, &account);
                                storage::remove_role(env, &account);
                            }
                            RoleType::Verifier => {
                                storage::revoke_verifier(env, &account);
                            }
                            RoleType::Trader => {
                                storage::revoke_trader(env, &account);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

#[contract]
pub struct RbacContract;

#[contractimpl]
impl RbacContract {
    /// Initializes the contract with the initial list of SuperAdmins, threshold, and timelock delay.
    pub fn initialize(
        env: Env,
        admins: Vec<Address>,
        threshold: u32,
        timelock_delay: u64,
    ) -> Result<(), Error> {
        if storage::is_initialized(&env) {
            return Err(Error::AlreadyInitialized);
        }
        if admins.is_empty() {
            return Err(Error::EmptyAdmins);
        }
        if threshold == 0 || threshold > admins.len() {
            return Err(Error::InvalidThreshold);
        }

        storage::set_initialized(&env);
        storage::write_super_admins(&env, &admins);
        storage::write_super_admin_threshold(&env, threshold);
        storage::write_timelock_delay(&env, timelock_delay);

        for admin in admins.iter() {
            storage::write_admin(&env, &admin);
            storage::write_role(&env, &admin, RoleType::SuperAdmin);
        }

        Ok(())
    }

    // --- Governance Proposal API ---

    /// Proposes a multi-sig action.
    /// Only a SuperAdmin can call this.
    pub fn propose_action(
        env: Env,
        proposer: Address,
        action: ProposalAction,
    ) -> Result<u64, Error> {
        proposer.require_auth();
        if !storage::is_super_admin(&env, &proposer) {
            return Err(Error::Unauthorized);
        }

        let next_id = storage::read_next_proposal_id(&env);
        let created_at = env.ledger().timestamp();

        let mut approvals = Vec::new(&env);
        approvals.push_back(proposer.clone());

        let mut proposal = Proposal {
            id: next_id,
            action: action.clone(),
            proposer: proposer.clone(),
            approvals,
            created_at,
            executed: false,
            rejected: false,
        };

        storage::write_next_proposal_id(&env, next_id + 1);

        let threshold = storage::read_super_admin_threshold(&env);
        let timelock_delay = storage::read_timelock_delay(&env);

        if threshold == 1 && timelock_delay == 0 {
            execute_proposal_logic(&env, &action)?;
            proposal.executed = true;
        }

        storage::write_proposal(&env, next_id, &proposal);

        Ok(next_id)
    }

    /// Approves an existing proposal.
    /// Only a SuperAdmin can call this.
    pub fn approve_proposal(
        env: Env,
        approver: Address,
        proposal_id: u64,
    ) -> Result<(), Error> {
        approver.require_auth();
        if !storage::is_super_admin(&env, &approver) {
            return Err(Error::Unauthorized);
        }

        let mut proposal = storage::read_proposal(&env, proposal_id).ok_or(Error::ProposalNotFound)?;

        if proposal.executed {
            return Err(Error::ProposalAlreadyExecuted);
        }
        if proposal.rejected {
            return Err(Error::ProposalRejected);
        }

        for app in proposal.approvals.iter() {
            if app == approver {
                return Err(Error::AlreadyApproved);
            }
        }

        proposal.approvals.push_back(approver);
        storage::write_proposal(&env, proposal_id, &proposal);

        Ok(())
    }

    /// Executes an approved proposal after the timelock has expired.
    /// Can be called by anyone.
    pub fn execute_proposal(
        env: Env,
        proposal_id: u64,
    ) -> Result<(), Error> {
        let mut proposal = storage::read_proposal(&env, proposal_id).ok_or(Error::ProposalNotFound)?;

        if proposal.executed {
            return Err(Error::ProposalAlreadyExecuted);
        }
        if proposal.rejected {
            return Err(Error::ProposalRejected);
        }

        let threshold = storage::read_super_admin_threshold(&env);
        if proposal.approvals.len() < threshold {
            return Err(Error::InsufficientApprovals);
        }

        let timelock_delay = storage::read_timelock_delay(&env);
        if env.ledger().timestamp() < proposal.created_at + timelock_delay {
            return Err(Error::TimelockNotExpired);
        }

        execute_proposal_logic(&env, &proposal.action)?;

        proposal.executed = true;
        storage::write_proposal(&env, proposal_id, &proposal);

        Ok(())
    }

    /// Vetoes/cancels a proposal.
    /// Only a SuperAdmin can call this.
    pub fn reject_proposal(
        env: Env,
        rejecter: Address,
        proposal_id: u64,
    ) -> Result<(), Error> {
        rejecter.require_auth();
        if !storage::is_super_admin(&env, &rejecter) {
            return Err(Error::Unauthorized);
        }

        let mut proposal = storage::read_proposal(&env, proposal_id).ok_or(Error::ProposalNotFound)?;

        if proposal.executed {
            return Err(Error::ProposalAlreadyExecuted);
        }
        if proposal.rejected {
            return Err(Error::ProposalRejected);
        }

        proposal.rejected = true;
        storage::write_proposal(&env, proposal_id, &proposal);

        Ok(())
    }

    /// Returns a proposal by ID.
    pub fn get_proposal(env: Env, proposal_id: u64) -> Option<Proposal> {
        storage::read_proposal(&env, proposal_id)
    }

    // --- Role Management ---

    pub fn grant_admin(env: Env, admin: Address, account: Address) -> Result<(), Error> {
        admin.require_auth();
        if !storage::is_admin(&env, &admin) {
            return Err(Error::Unauthorized);
        }
        storage::write_admin(&env, &account);
        storage::write_role(&env, &account, RoleType::Admin);

        Ok(())
    }

    pub fn grant_verifier(env: Env, admin: Address, account: Address) -> Result<(), Error> {
        admin.require_auth();
        if !storage::is_admin(&env, &admin) {
            return Err(Error::Unauthorized);
        }
        storage::write_role(&env, &account, RoleType::Verifier);

        Ok(())
    }

    pub fn grant_trader(env: Env, admin: Address, account: Address) -> Result<(), Error> {
        admin.require_auth();
        if !storage::is_admin(&env, &admin) {
            return Err(Error::Unauthorized);
        }
        storage::write_role(&env, &account, RoleType::Trader);

        Ok(())
    }

    pub fn revoke_role(env: Env, admin: Address, account: Address) -> Result<(), Error> {
        admin.require_auth();
        if !storage::is_admin(&env, &admin) {
            return Err(Error::Unauthorized);
        }

        if storage::is_super_admin(&env, &account) {
            return Err(Error::CannotRemoveSuperAdmin);
        }

        match storage::read_role(&env, &account) {
            Some(RoleType::Admin) => {
                storage::revoke_admin(&env, &account);
                storage::remove_role(&env, &account);
                Ok(())
            }
            Some(RoleType::Verifier) => {
                storage::revoke_verifier(&env, &account);
                Ok(())
            }
            Some(RoleType::Trader) => {
                storage::revoke_trader(&env, &account);
                Ok(())
            }
            Some(RoleType::SuperAdmin) => Err(Error::CannotRemoveSuperAdmin),
            None => Err(Error::RoleNotAssigned),
        }
    }

    // --- Batch Operations (Redirected or single-admin only) ---

    pub fn assign_roles_batch(
        env: Env,
        super_admin: Address,
        assignments: Vec<(Address, Symbol)>,
    ) -> Result<(), Error> {
        let threshold = storage::read_super_admin_threshold(&env);
        if threshold > 1 {
            return Err(Error::Unauthorized); // Must go through proposal flow when threshold > 1
        }

        super_admin.require_auth();
        if !storage::is_super_admin(&env, &super_admin) {
            return Err(Error::Unauthorized);
        }

        execute_proposal_logic(&env, &ProposalAction::AssignRolesBatch(assignments))
    }

    pub fn revoke_roles_batch(
        env: Env,
        super_admin: Address,
        revocations: Vec<(Address, Symbol)>,
    ) -> Result<(), Error> {
        let threshold = storage::read_super_admin_threshold(&env);
        if threshold > 1 {
            return Err(Error::Unauthorized); // Must go through proposal flow when threshold > 1
        }

        super_admin.require_auth();
        if !storage::is_super_admin(&env, &super_admin) {
            return Err(Error::Unauthorized);
        }

        execute_proposal_logic(&env, &ProposalAction::RevokeRolesBatch(revocations))
    }

    // --- Role Checks ---

    pub fn has_role(env: Env, account: Address, role: String) -> bool {
        let role_type = if role == String::from_str(&env, "Admin") {
            RoleType::Admin
        } else if role == String::from_str(&env, "Verifier") {
            RoleType::Verifier
        } else if role == String::from_str(&env, "Trader") {
            RoleType::Trader
        } else if role == String::from_str(&env, "SuperAdmin") {
            RoleType::SuperAdmin
        } else {
            return false;
        };

        if let Some(assigned) = storage::read_role(&env, &account) {
            assigned == role_type
        } else {
            false
        }
    }

    pub fn is_admin(env: Env, account: Address) -> bool {
        storage::is_admin(&env, &account)
    }

    pub fn transfer_admin(env: Env, new_admin: Address) -> Result<(), Error> {
        let admins = storage::read_super_admins(&env);
        let threshold = storage::read_super_admin_threshold(&env);

        if threshold > 1 {
            return Err(Error::Unauthorized); // Must go through proposal flow when threshold > 1
        }

        let caller = admins.get(0).ok_or(Error::Unauthorized)?;
        caller.require_auth();

        // Clean up old super admin roles
        for old_admin in admins.iter() {
            if old_admin != new_admin {
                storage::revoke_admin(&env, &old_admin);
                storage::remove_role(&env, &old_admin);
            }
        }

        // Set new single super admin
        let mut new_admins = Vec::new(&env);
        new_admins.push_back(new_admin.clone());
        storage::write_super_admins(&env, &new_admins);
        storage::write_super_admin_threshold(&env, 1);

        storage::write_role(&env, &new_admin, RoleType::SuperAdmin);
        storage::write_admin(&env, &new_admin);

        Ok(())
    }

    pub fn get_super_admin(env: Env) -> Address {
        storage::read_super_admin(&env)
    }

    pub fn get_role(env: Env, address: Address) -> u32 {
        match storage::read_role(&env, &address) {
            Some(RoleType::SuperAdmin) => 0,
            Some(RoleType::Verifier) => 1,
            Some(RoleType::Trader) => 2,
            Some(RoleType::Admin) => 3,
            None => 255,
        }
    }
}
