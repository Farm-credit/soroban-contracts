#![cfg(test)]

use crate::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Bytes, Env, String,
};

fn create_token<'a>(e: &Env, admin: &Address) -> CarbonCreditTokenClient<'a> {
    let contract_id = e.register_contract(None, CarbonCreditToken);
    let client = CarbonCreditTokenClient::new(e, &contract_id);

    client.initialize(
        admin,
        &String::from_str(e, "Carbon Credit Token"),
        &String::from_str(e, "CCT"),
        &0u32,
        &String::from_str(e, "Amazon Reforestation"),
        &String::from_str(e, "2023"),
        &String::from_str(e, "Brazil"),
        &String::from_str(e, "https://farmcredit.xyz/amazon-1"),
    );

    client
}

// ============ INITIALIZATION TESTS ============

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token = create_token(&env, &admin);

    assert_eq!(token.name(), String::from_str(&env, "Carbon Credit Token"));
    assert_eq!(token.symbol(), String::from_str(&env, "CCT"));
    assert_eq!(token.decimals(), 0u32);
    assert_eq!(token.total_supply(), 0i128);
    assert_eq!(token.total_retired(), 0i128);
}

// ============ MINT TESTS ============

#[test]
fn test_mint() {
    let env = Env::default();
    env.mock_all_auths();

    let rbac_id = env.register_contract(None, MockRbacContract);
    let token_id = env.register_contract(None, CarbonCreditToken);
    let token = CarbonCreditTokenClient::new(&env, &token_id);

    let admin = Address::generate(&env);
    let verifier = Address::generate(&env);
    let user = Address::generate(&env);

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
    let (env, token, _, verifier, user) = setup();

    let hash1 = Bytes::from_slice(&env, b"report_hash_1234");
    let hash2 = Bytes::from_slice(&env, b"report_hash_5678");
    let hash3 = Bytes::from_slice(&env, b"report_hash_9999");
    let methodology = String::from_str(&env, "VCS");

    // Mint some tokens
    token.mint(&verifier, &user, &1000, &hash1);
    assert_eq!(token.balance(&user), 1000);

    // Retire some tokens
    token.retire(&user, &300, &hash2, &methodology);

    assert_eq!(token.balance(&user), 700);
    assert_eq!(token.total_retired(), 300);

    // Verify NFT creation
    assert_eq!(token.certificate_count(), 1);
    let cert = token.get_certificate(&1).unwrap();
    assert_eq!(cert.owner, user1);
    assert_eq!(cert.amount, 300);
    assert_eq!(cert.project_name, String::from_str(&env, "Amazon Reforestation"));
    assert_eq!(cert.vintage, String::from_str(&env, "2023"));
    assert_eq!(cert.location, String::from_str(&env, "Brazil"));
    assert_eq!(cert.metadata_url, String::from_str(&env, "https://farmcredit.xyz/amazon-1"));
}

#[test]
fn test_retire_multiple_times() {
    let env = Env::default();
    env.mock_all_auths();

    let cert = certs.get(0).unwrap();
    assert_eq!(cert.id, 1);
    assert_eq!(cert.amount, 300);
    assert!(cert.timestamp > 0);

    assert_eq!(token.get_certificate_count(), 1);

    // Retire more
    token.retire(&user, &200, &hash3, &methodology);
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
