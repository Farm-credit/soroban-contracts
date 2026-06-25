use soroban_sdk::{contracttype, Address, Env, Vec, Symbol};

// ── TTL Constants (standardized across all contracts) ────────────────────────
pub const INSTANCE_LIFETIME_THRESHOLD: u32 = 17280; // ~1 day at 5s/ledger
pub const INSTANCE_BUMP_AMOUNT: u32 = 518400; // ~30 days at 5s/ledger

// ── Role Types ───────────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq, Debug, Eq)]
#[contracttype]
pub enum RoleType {
    SuperAdmin,
    Admin,
    Verifier,
    Trader,
}

// ── Governance Types ─────────────────────────────────────────────────────────
#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub enum ProposalAction {
    ChangeSuperAdmins { admins: Vec<Address>, threshold: u32 },
    SetTimelockDelay(u64),
    GrantAdmin(Address),
    RevokeAdmin(Address),
    AssignRolesBatch(Vec<(Address, Symbol)>),
    RevokeRolesBatch(Vec<(Address, Symbol)>),
}

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub struct Proposal {
    pub id: u64,
    pub action: ProposalAction,
    pub proposer: Address,
    pub approvals: Vec<Address>,
    pub created_at: u64,
    pub executed: bool,
    pub rejected: bool,
}

// ── Storage Keys ─────────────────────────────────────────────────────────────
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Initialized,
    SuperAdmins,
    SuperAdminThreshold,
    TimelockDelay,
    Admin(Address), // Map of addresses with Admin role
    Role(Address),
    NextProposalId,
    Proposal(u64),
}

// ── Initialization ────────────────────────────────────────────────────────────
pub fn is_initialized(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Initialized)
}

pub fn set_initialized(e: &Env) {
    e.storage().instance().set(&DataKey::Initialized, &true);
}

// ── SuperAdmins List ─────────────────────────────────────────────────────────
pub fn read_super_admins(e: &Env) -> Vec<Address> {
    e.storage()
        .instance()
        .get(&DataKey::SuperAdmins)
        .expect("super admins not set")
}

pub fn write_super_admins(e: &Env, admins: &Vec<Address>) {
    e.storage().instance().set(&DataKey::SuperAdmins, admins);
}

pub fn read_super_admin_threshold(e: &Env) -> u32 {
    e.storage()
        .instance()
        .get(&DataKey::SuperAdminThreshold)
        .unwrap_or(1)
}

pub fn write_super_admin_threshold(e: &Env, threshold: u32) {
    e.storage()
        .instance()
        .set(&DataKey::SuperAdminThreshold, &threshold);
}

pub fn read_timelock_delay(e: &Env) -> u64 {
    e.storage()
        .instance()
        .get(&DataKey::TimelockDelay)
        .unwrap_or(0)
}

pub fn write_timelock_delay(e: &Env, delay: u64) {
    e.storage().instance().set(&DataKey::TimelockDelay, &delay);
}

// ── Proposals ─────────────────────────────────────────────────────────────────
pub fn read_next_proposal_id(e: &Env) -> u64 {
    e.storage()
        .instance()
        .get(&DataKey::NextProposalId)
        .unwrap_or(1)
}

pub fn write_next_proposal_id(e: &Env, id: u64) {
    e.storage().instance().set(&DataKey::NextProposalId, &id);
}

pub fn read_proposal(e: &Env, id: u64) -> Option<Proposal> {
    e.storage().persistent().get(&DataKey::Proposal(id))
}

pub fn write_proposal(e: &Env, id: u64, proposal: &Proposal) {
    e.storage().persistent().set(&DataKey::Proposal(id), proposal);
}

// ── SuperAdmin (Legacy Compatibility) ─────────────────────────────────────────
pub fn read_super_admin(e: &Env) -> Address {
    let admins = read_super_admins(e);
    admins.get(0).expect("super admin not set")
}

pub fn write_super_admin(e: &Env, admin: &Address) {
    let mut admins = Vec::new(e);
    admins.push_back(admin.clone());
    write_super_admins(e, &admins);
    write_super_admin_threshold(e, 1);
}

// ── Role Helpers ──────────────────────────────────────────────────────────────
pub fn read_role(e: &Env, address: &Address) -> Option<RoleType> {
    e.storage()
        .persistent()
        .get(&DataKey::Role(address.clone()))
}

pub fn write_role(e: &Env, address: &Address, role: RoleType) {
    e.storage()
        .persistent()
        .set(&DataKey::Role(address.clone()), &role);
}

pub fn remove_role(e: &Env, address: &Address) {
    e.storage()
        .persistent()
        .remove(&DataKey::Role(address.clone()));
}

// ── Role Write Helpers ───────────────────────────────────────────────────────
pub fn write_admin(e: &Env, address: &Address) {
    e.storage()
        .instance()
        .set(&DataKey::Admin(address.clone()), &true);
}

pub fn revoke_admin(e: &Env, address: &Address) {
    e.storage()
        .instance()
        .remove(&DataKey::Admin(address.clone()));
}

pub fn revoke_verifier(e: &Env, address: &Address) {
    remove_role(e, address);
}

pub fn revoke_trader(e: &Env, address: &Address) {
    remove_role(e, address);
}

// ── Role Checks ───────────────────────────────────────────────────────────────
pub fn is_super_admin(e: &Env, address: &Address) -> bool {
    matches!(read_role(e, address), Some(RoleType::SuperAdmin))
}

pub fn is_admin(e: &Env, address: &Address) -> bool {
    matches!(
        read_role(e, address),
        Some(RoleType::SuperAdmin) | Some(RoleType::Admin)
    )
}

