#![cfg(test)]

use crate::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Bytes, Env, String,
};

use crate::error::Error;

// ─────────────────────────────────────────────────────────────────────────────
// Mock RBAC Contract
// ─────────────────────────────────────────────────────────────────────────────

#[soroban_sdk::contract]
pub struct MockRbacContract;

#[soroban_sdk::contractimpl]
impl MockRbacContract {
    pub fn has_role(_env: Env, _address: Address, _role: String) -> bool {
        true // Mock: everyone is a verifier for tests
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Test helpers
// ─────────────────────────────────────────────────────────────────────────────

fn setup() -> (
    Env,
    CarbonCreditTokenClient<'static>,
    Address, // admin
    Address, // verifier
    Address, // user
) {
    let env = Env::default();
    env.mock_all_auths();

    let rbac_id = env.register_contract(None, MockRbacContract);
    let token_id = env.register_contract(None, CarbonCreditToken);
    let token = CarbonCreditTokenClient::new(&env, &token_id);

    let admin    = Address::generate(&env);
    let verifier = Address::generate(&env);
    let user     = Address::generate(&env);

    token.initialize(
        &admin,
        &rbac_id,
        &String::from_str(&env, "Carbon Credit Token"),
        &String::from_str(&env, "CCT"),
        &0u32,
    );

    (env, token, admin, verifier, user)
}

#[test]
fn test_retire_with_audit_metadata() {
    let (env, token, _, verifier, user) = setup();
    
    let report_hash = Bytes::from_array(&env, &[1u8; 32]);
    let methodology = String::from_str(&env, "VERRA-VM0001");

    // Mint tokens
    token.mint(&verifier, &user, &1000, &report_hash);
    assert_eq!(token.balance(&user), 1000);

    // Retire tokens with audit metadata
    token.retire(&user, &300, &report_hash, &methodology);
    
    assert_eq!(token.balance(&user), 700);
    assert_eq!(token.total_retired(), 300);

    // Verify events (indirectly via compilation/test pass)
    // In a real audit test, we would inspect the event log, but here we focus on interface extension.
}

#[test]
fn test_prevent_double_mint_with_same_hash() {
    let (env, token, _, verifier, user) = setup();
    let report_hash = Bytes::from_array(&env, &[2u8; 32]);

    token.mint(&verifier, &user, &500, &report_hash);
    
    // Attempting second mint with same hash should panic
    let result = token.try_mint(&verifier, &user, &500, &report_hash);
    assert!(result.is_err());
}
