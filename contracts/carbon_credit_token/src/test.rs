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

    env.ledger().with_mut(|l| l.timestamp = 123456789);

    (env, token, admin, verifier, user)
}

#[test]
fn test_retire_and_certificate_issuance() {
    let (_env, token, _, verifier, user) = setup();

    
    // Mint some tokens
    token.mint(&verifier, &user, &1000);
    assert_eq!(token.balance(&user), 1000);

    // Retire some tokens
    token.retire(&user, &300);
    
    assert_eq!(token.balance(&user), 700);
    assert_eq!(token.total_retired(), 300);

    // Check certificate issuance
    let certs = token.get_certificates(&user);
    assert_eq!(certs.len(), 1);
    
    let cert = certs.get(0).unwrap();
    assert_eq!(cert.id, 1);
    assert_eq!(cert.amount, 300);
    assert!(cert.timestamp > 0);
    
    assert_eq!(token.get_certificate_count(), 1);

    // Retire more
    token.retire(&user, &200);
    let certs2 = token.get_certificates(&user);
    assert_eq!(certs2.len(), 2);
    assert_eq!(certs2.get(1).unwrap().amount, 200);
    assert_eq!(token.get_certificate_count(), 2);
}

#[test]
fn test_view_certificates_empty() {
    let (env, token, _, _, _) = setup();
    let nobody = Address::generate(&env);
    let certs = token.get_certificates(&nobody);
    assert_eq!(certs.len(), 0);
}

// ── Pause / unpause ───────────────────────────────────────────────────────────

#[test]
fn test_paused_initially_false() {
    let (_env, token, _, _, _) = setup();
    assert!(!token.paused());
}

#[test]
fn test_admin_pause_and_unpause() {
    let (_env, token, admin, _, _) = setup();
    token.admin_pause(&admin);
    assert!(token.paused());
    token.admin_unpause(&admin);
    assert!(!token.paused());
}

#[test]
fn test_pause_emits_event() {
    let (env, token, admin, _, _) = setup();
    let before = env.events().all().len();
    token.admin_pause(&admin);
    assert!(env.events().all().len() > before);
}

#[test]
fn test_unpause_emits_event() {
    let (env, token, admin, _, _) = setup();
    token.admin_pause(&admin);
    let before = env.events().all().len();
    token.admin_unpause(&admin);
    assert!(env.events().all().len() > before);
}

#[test]
fn test_pause_by_non_admin_returns_error() {
    let (env, token, _, _, _) = setup();
    let rando = Address::generate(&env);
    let result = token.try_admin_pause(&rando);
    assert!(result.is_err());
}

#[test]
fn test_mint_when_paused_returns_error() {
    let (_env, token, admin, verifier, user) = setup();
    token.admin_pause(&admin);
    let result = token.try_mint(&verifier, &user, &1000);
    assert_eq!(result, Err(Ok(Error::ContractPaused)));
}

#[test]
fn test_transfer_when_paused_returns_error() {
    let (env, token, admin, verifier, user) = setup();
    token.mint(&verifier, &user, &500);
    token.admin_pause(&admin);
    let other = Address::generate(&env);
    let result = token.try_transfer(&user, &other, &100);
    assert_eq!(result, Err(Ok(Error::ContractPaused)));
}

#[test]
fn test_burn_when_paused_returns_error() {
    let (_env, token, admin, verifier, user) = setup();
    token.mint(&verifier, &user, &500);
    token.admin_pause(&admin);
    let result = token.try_burn(&user, &100);
    assert_eq!(result, Err(Ok(Error::ContractPaused)));
}

#[test]
fn test_operations_resume_after_unpause() {
    let (env, token, admin, verifier, user) = setup();
    token.admin_pause(&admin);
    token.admin_unpause(&admin);
    // should succeed after unpause
    token.mint(&verifier, &user, &500);
    assert_eq!(token.balance(&user), 500);
    let other = Address::generate(&env);
    token.transfer(&user, &other, &200);
    assert_eq!(token.balance(&other), 200);
}
