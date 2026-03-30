#![cfg(test)]

use crate::{EscrowContract, Offer};
use soroban_sdk::{testutils::Address as _, Address, Env};

fn create_escrow<'a>(env: &Env) -> (crate::EscrowContractClient<'a>, Address) {
    let contract_id = env.register_contract(None, EscrowContract);
    let client = crate::EscrowContractClient::new(env, &contract_id);
    client.initialize();
    (client, contract_id)
}

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();
    let (_client, _contract_id) = create_escrow(&env);
}

#[test]
#[should_panic(expected = "escrow already initialized")]
fn test_initialize_twice_panics() {
    let env = Env::default();
    env.mock_all_auths();

    let (client, _contract_id) = create_escrow(&env);
    client.initialize();
}

#[test]
#[should_panic(expected = "amounts must be positive")]
fn test_create_offer_zero_carbon_amount_panics() {
    let env = Env::default();
    env.mock_all_auths();

    let seller = Address::generate(&env);
    let carbon_token = Address::generate(&env);
    let usdc_token = Address::generate(&env);
    let (client, _contract_id) = create_escrow(&env);

    client.create_offer(&seller, &0i128, &1000i128, &carbon_token, &usdc_token);
}

#[test]
#[should_panic(expected = "amounts must be positive")]
fn test_create_offer_zero_usdc_amount_panics() {
    let env = Env::default();
    env.mock_all_auths();

    let seller = Address::generate(&env);
    let carbon_token = Address::generate(&env);
    let usdc_token = Address::generate(&env);
    let (client, _contract_id) = create_escrow(&env);

    client.create_offer(&seller, &100i128, &0i128, &carbon_token, &usdc_token);
}

#[test]
#[should_panic(expected = "amounts must be positive")]
fn test_create_offer_negative_carbon_amount_panics() {
    let env = Env::default();
    env.mock_all_auths();

    let seller = Address::generate(&env);
    let carbon_token = Address::generate(&env);
    let usdc_token = Address::generate(&env);
    let (client, _contract_id) = create_escrow(&env);

    client.create_offer(&seller, &-100i128, &1000i128, &carbon_token, &usdc_token);
}

#[test]
#[should_panic(expected = "offer not found")]
fn test_fill_nonexistent_offer_panics() {
    let env = Env::default();
    env.mock_all_auths();

    let buyer = Address::generate(&env);
    let (client, _contract_id) = create_escrow(&env);
    client.fill_offer(&999u64, &buyer, &100i128);
}

#[test]
#[should_panic(expected = "offer not found")]
fn test_cancel_nonexistent_offer_panics() {
    let env = Env::default();
    env.mock_all_auths();

    let caller = Address::generate(&env);
    let (client, _contract_id) = create_escrow(&env);
    client.cancel_offer(&999u64, &caller);
}

#[test]
fn test_get_nonexistent_offer_returns_none() {
    let env = Env::default();
    env.mock_all_auths();

    let (client, _contract_id) = create_escrow(&env);
    let offer = client.get_offer(&999u64);
    assert!(offer.is_none());
}

#[test]
fn test_get_remaining_amount_nonexistent_offer_returns_zeroes() {
    let env = Env::default();
    env.mock_all_auths();

    let (client, _contract_id) = create_escrow(&env);
    let (carbon, usdc) = client.get_remaining_amount(&999u64);
    assert_eq!(carbon, 0i128);
    assert_eq!(usdc, 0i128);
}

#[test]
fn test_offer_remaining_carbon_and_usdc() {
    let env = Env::default();
    let offer = Offer {
        offer_id: 1,
        seller: Address::generate(&env),
        carbon_amount: 1000,
        usdc_amount: 5000,
        filled_carbon: 300,
        filled_usdc: 1500,
        carbon_token: Address::generate(&env),
        usdc_token: Address::generate(&env),
        is_cancelled: false,
    };

    assert_eq!(offer.remaining_carbon(), 700);
    assert_eq!(offer.remaining_usdc(), 3500);
}

#[test]
fn test_offer_is_fully_filled() {
    let env = Env::default();

    let pending = Offer {
        offer_id: 1,
        seller: Address::generate(&env),
        carbon_amount: 1000,
        usdc_amount: 5000,
        filled_carbon: 999,
        filled_usdc: 4995,
        carbon_token: Address::generate(&env),
        usdc_token: Address::generate(&env),
        is_cancelled: false,
    };
    assert!(!pending.is_fully_filled());
    assert!(pending.is_active());

    let filled = Offer {
        offer_id: 2,
        seller: Address::generate(&env),
        carbon_amount: 1000,
        usdc_amount: 5000,
        filled_carbon: 1000,
        filled_usdc: 5000,
        carbon_token: Address::generate(&env),
        usdc_token: Address::generate(&env),
        is_cancelled: false,
    };
    assert!(filled.is_fully_filled());
    assert!(!filled.is_active());
}

#[test]
fn test_view_functions_on_empty_state() {
    let env = Env::default();
    env.mock_all_auths();

    let seller = Address::generate(&env);
    let carbon_token = Address::generate(&env);
    let usdc_token = Address::generate(&env);
    let (client, _contract_id) = create_escrow(&env);

    assert_eq!(client.get_offer_count(), 0u64);

    let active = client.get_active_offers(&0u64, &10u64);
    assert_eq!(active.len(), 0);

    let by_seller = client.get_offers_by_seller(&seller);
    assert_eq!(by_seller.len(), 0);

    let by_pair = client.get_offers_by_token_pair(&carbon_token, &usdc_token, &0u64, &10u64);
    assert_eq!(by_pair.len(), 0);
}
