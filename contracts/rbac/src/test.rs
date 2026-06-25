#![cfg(test)]

use soroban_sdk::{testutils::{Address as _, Ledger}, Address, Env, String, Symbol, vec};

use crate::{error::Error, RbacContract, RbacContractClient};
use crate::storage::{ProposalAction, RoleType};

fn setup_single(env: &Env) -> (RbacContractClient<'static>, Address) {
    let contract_id = env.register_contract(None, RbacContract);
    let client = RbacContractClient::new(env, &contract_id);

    let super_admin = Address::generate(env);
    let admins = vec![env, super_admin.clone()];
    client.initialize(&admins, &1u32, &0u64);

    (client, super_admin)
}

fn setup_multi(env: &Env, threshold: u32, delay: u64) -> (RbacContractClient<'static>, soroban_sdk::Vec<Address>) {
    let contract_id = env.register_contract(None, RbacContract);
    let client = RbacContractClient::new(env, &contract_id);

    let admin1 = Address::generate(env);
    let admin2 = Address::generate(env);
    let admin3 = Address::generate(env);
    let admins = vec![env, admin1.clone(), admin2.clone(), admin3.clone()];
    client.initialize(&admins, &threshold, &delay);

    (client, admins)
}

// ── Initialization Tests ──────────────────────────────────────────────────────

#[test]
fn test_initialize_sets_super_admins() {
    let env = Env::default();
    let (client, super_admin) = setup_single(&env);
    assert_eq!(client.get_super_admin(), super_admin);
    assert!(client.has_role(&super_admin, &String::from_str(&env, "SuperAdmin")));
    assert!(client.is_admin(&super_admin));
    assert_eq!(client.get_role(&super_admin), 0u32);
}

#[test]
fn test_initialize_twice_fails() {
    let env = Env::default();
    let (client, super_admin) = setup_single(&env);
    let other = Address::generate(&env);
    let admins = vec![&env, other];
    let result = client.try_initialize(&admins, &1u32, &0u64);
    assert_eq!(result, Err(Ok(Error::AlreadyInitialized)));
}

#[test]
fn test_initialize_invalid_threshold() {
    let env = Env::default();
    let contract_id = env.register_contract(None, RbacContract);
    let client = RbacContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let admins = vec![&env, admin];
    
    // Threshold = 0 fails
    let res1 = client.try_initialize(&admins, &0u32, &0u64);
    assert_eq!(res1, Err(Ok(Error::InvalidThreshold)));

    // Threshold > admins.len() fails
    let res2 = client.try_initialize(&admins, &2u32, &0u64);
    assert_eq!(res2, Err(Ok(Error::InvalidThreshold)));
}

// ── Role Management Tests (Single Admin Mode) ──────────────────────────────────

#[test]
fn test_role_management_single_admin() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, super_admin) = setup_single(&env);

    let verifier = Address::generate(&env);
    let trader = Address::generate(&env);
    let admin = Address::generate(&env);

    // Grant roles
    client.grant_verifier(&super_admin, &verifier);
    client.grant_trader(&super_admin, &trader);
    client.grant_admin(&super_admin, &admin);

    assert!(client.has_role(&verifier, &String::from_str(&env, "Verifier")));
    assert!(client.has_role(&trader, &String::from_str(&env, "Trader")));
    assert!(client.has_role(&admin, &String::from_str(&env, "Admin")));

    assert_eq!(client.get_role(&verifier), 1u32);
    assert_eq!(client.get_role(&trader), 2u32);
    assert_eq!(client.get_role(&admin), 3u32);

    // Revoke role
    client.revoke_role(&super_admin, &verifier);
    assert!(!client.has_role(&verifier, &String::from_str(&env, "Verifier")));

    // SuperAdmin cannot be revoked directly
    let res = client.try_revoke_role(&super_admin, &super_admin);
    assert_eq!(res, Err(Ok(Error::CannotRemoveSuperAdmin)));
}

// ── Batch Operations (Single Admin Mode) ──────────────────────────────────────

#[test]
fn test_batch_operations_single_admin() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, super_admin) = setup_single(&env);

    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    let assignments = vec![
        &env,
        (user1.clone(), Symbol::new(&env, "Verifier")),
        (user2.clone(), Symbol::new(&env, "Trader")),
    ];

    client.assign_roles_batch(&super_admin, &assignments);
    assert!(client.has_role(&user1, &String::from_str(&env, "Verifier")));
    assert!(client.has_role(&user2, &String::from_str(&env, "Trader")));

    let revocations = vec![
        &env,
        (user1.clone(), Symbol::new(&env, "Verifier")),
        (user2.clone(), Symbol::new(&env, "Trader")),
    ];
    client.revoke_roles_batch(&super_admin, &revocations);
    assert!(!client.has_role(&user1, &String::from_str(&env, "Verifier")));
    assert!(!client.has_role(&user2, &String::from_str(&env, "Trader")));
}

#[test]
fn test_transfer_admin_single_admin() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, super_admin) = setup_single(&env);

    let new_admin = Address::generate(&env);
    client.transfer_admin(&new_admin);

    assert_eq!(client.get_super_admin(), new_admin);
    assert!(client.has_role(&new_admin, &String::from_str(&env, "SuperAdmin")));
    assert!(!client.has_role(&super_admin, &String::from_str(&env, "SuperAdmin")));
}

// ── Multi-Sig Proposal Flow Tests ─────────────────────────────────────────────

#[test]
fn test_multi_sig_immediate_execution_on_threshold_1() {
    let env = Env::default();
    env.mock_all_auths();
    
    // threshold = 1, delay = 0
    let (client, admins) = setup_multi(&env, 1, 0);
    let admin1 = admins.get(0).unwrap();
    let user = Address::generate(&env);

    let action = ProposalAction::GrantAdmin(user.clone());
    let proposal_id = client.propose_action(&admin1, &action);

    // Should be executed immediately
    let proposal = client.get_proposal(&proposal_id).unwrap();
    assert!(proposal.executed);
    assert!(client.is_admin(&user));
}

#[test]
fn test_multi_sig_proposal_and_approval_threshold_2() {
    let env = Env::default();
    env.mock_all_auths();
    
    // threshold = 2, delay = 0
    let (client, admins) = setup_multi(&env, 2, 0);
    let admin1 = admins.get(0).unwrap();
    let admin2 = admins.get(1).unwrap();
    let user = Address::generate(&env);

    let action = ProposalAction::GrantAdmin(user.clone());
    
    // Propose (adds 1st approval from admin1)
    let proposal_id = client.propose_action(&admin1, &action);
    let mut proposal = client.get_proposal(&proposal_id).unwrap();
    assert!(!proposal.executed);
    assert_eq!(proposal.approvals.len(), 1);

    // Try executing directly - fails (insufficient approvals)
    let res_exec = client.try_execute_proposal(&proposal_id);
    assert_eq!(res_exec, Err(Ok(Error::InsufficientApprovals)));

    // Approve by same admin fails
    let res_dup = client.try_approve_proposal(&admin1, &proposal_id);
    assert_eq!(res_dup, Err(Ok(Error::AlreadyApproved)));

    // Approve by admin2 (adds 2nd approval)
    client.approve_proposal(&admin2, &proposal_id);
    proposal = client.get_proposal(&proposal_id).unwrap();
    assert_eq!(proposal.approvals.len(), 2);

    // Execute proposal
    client.execute_proposal(&proposal_id);
    proposal = client.get_proposal(&proposal_id).unwrap();
    assert!(proposal.executed);
    assert!(client.is_admin(&user));
}

#[test]
fn test_multi_sig_rejection() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, admins) = setup_multi(&env, 2, 0);
    let admin1 = admins.get(0).unwrap();
    let admin2 = admins.get(1).unwrap();
    let user = Address::generate(&env);

    let action = ProposalAction::GrantAdmin(user.clone());
    let proposal_id = client.propose_action(&admin1, &action);

    // Reject proposal
    client.reject_proposal(&admin2, &proposal_id);
    let proposal = client.get_proposal(&proposal_id).unwrap();
    assert!(proposal.rejected);

    // Try approving rejected proposal fails
    let res_app = client.try_approve_proposal(&admin2, &proposal_id);
    assert_eq!(res_app, Err(Ok(Error::ProposalRejected)));

    // Try executing rejected proposal fails
    let res_exec = client.try_execute_proposal(&proposal_id);
    assert_eq!(res_exec, Err(Ok(Error::ProposalRejected)));
}

#[test]
fn test_multi_sig_timelock_delay() {
    let env = Env::default();
    env.mock_all_auths();
    
    // threshold = 2, delay = 100 seconds
    let (client, admins) = setup_multi(&env, 2, 100);
    let admin1 = admins.get(0).unwrap();
    let admin2 = admins.get(1).unwrap();
    let user = Address::generate(&env);

    // Set initial ledger time
    env.ledger().set_timestamp(1000);

    let action = ProposalAction::GrantAdmin(user.clone());
    let proposal_id = client.propose_action(&admin1, &action);
    client.approve_proposal(&admin2, &proposal_id);

    // Try executing immediately at t=1000 - fails (timelock not expired)
    let res_early = client.try_execute_proposal(&proposal_id);
    assert_eq!(res_early, Err(Ok(Error::TimelockNotExpired)));

    // Set time to t=1099 - still fails
    env.ledger().set_timestamp(1099);
    let res_almost = client.try_execute_proposal(&proposal_id);
    assert_eq!(res_almost, Err(Ok(Error::TimelockNotExpired)));

    // Set time to t=1100 - succeeds!
    env.ledger().set_timestamp(1100);
    client.execute_proposal(&proposal_id);
    
    let proposal = client.get_proposal(&proposal_id).unwrap();
    assert!(proposal.executed);
    assert!(client.is_admin(&user));
}

#[test]
fn test_change_super_admins_via_proposal() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, admins) = setup_multi(&env, 2, 0);
    let admin1 = admins.get(0).unwrap();
    let admin2 = admins.get(1).unwrap();

    let new_admin1 = Address::generate(&env);
    let new_admin2 = Address::generate(&env);
    let new_admins = vec![&env, new_admin1.clone(), new_admin2.clone()];

    let action = ProposalAction::ChangeSuperAdmins {
        admins: new_admins,
        threshold: 2,
    };

    let proposal_id = client.propose_action(&admin1, &action);
    client.approve_proposal(&admin2, &proposal_id);
    client.execute_proposal(&proposal_id);

    // Verify new super admins are active
    assert!(client.has_role(&new_admin1, &String::from_str(&env, "SuperAdmin")));
    assert!(client.has_role(&new_admin2, &String::from_str(&env, "SuperAdmin")));

    // Verify old super admins are revoked
    assert!(!client.has_role(&admin1, &String::from_str(&env, "SuperAdmin")));
}