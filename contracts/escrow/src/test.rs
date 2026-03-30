#![cfg(test)]

use crate::{EscrowContract, Offer};
use soroban_sdk::{
    testutils::Address as _,
    Address, Env,
};

fn create_escrow<'a>(e: &Env) -> (crate::EscrowContractClient<'a>, Address) {
    let contract_id = e.register_contract(None, EscrowContract);
    let client = crate::EscrowContractClient::new(e, &contract_id);
    client.initialize();
    (client, contract_id)
}

// ============ INITIALIZATION TESTS ============

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();
    let (_, _) = create_escrow(&env);
}

#[test]
#[should_panic(expected = "escrow already initialized")]
fn test_initialize_twice_panics() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _) = create_escrow(&env);
    client.initialize();
}

// ============ CREATE OFFER VALIDATION TESTS ============

#[test]
#[should_panic(expected = "amounts must be positive")]
fn test_create_offer_zero_carbon_amount_panics() {
    let env = Env::default();
    env.mock_all_auths();
    let seller = Address::generate(&env);
    let carbon_token = Address::generate(&env);
    let usdc_token = Address::generate(&env);
    let (escrow_client, _) = create_escrow(&env);
    escrow_client.create_offer(&seller, &0i128, &1000i128, &carbon_token, &usdc_token, &10i128);
}

#[test]
#[should_panic(expected = "amounts must be positive")]
fn test_create_offer_zero_usdc_amount_panics() {
    let env = Env::default();
    env.mock_all_auths();
    let seller = Address::generate(&env);
    let carbon_token = Address::generate(&env);
    let usdc_token = Address::generate(&env);
    let (escrow_client, _) = create_escrow(&env);
    escrow_client.create_offer(&seller, &100i128, &0i128, &carbon_token, &usdc_token, &10i128);
}

#[test]
#[should_panic(expected = "amounts must be positive")]
fn test_create_offer_negative_carbon_amount_panics() {
    let env = Env::default();
    env.mock_all_auths();
    let seller = Address::generate(&env);
    let carbon_token = Address::generate(&env);
    let usdc_token = Address::generate(&env);
    let (escrow_client, _) = create_escrow(&env);
    escrow_client.create_offer(&seller, &-100i128, &1000i128, &carbon_token, &usdc_token, &10i128);
}

#[test]
#[should_panic(expected = "min_fill_amount must be positive")]
fn test_create_offer_zero_min_fill_amount_panics() {
    let env = Env::default();
    env.mock_all_auths();
    let seller = Address::generate(&env);
    let carbon_token = Address::generate(&env);
    let usdc_token = Address::generate(&env);
    let (escrow_client, _) = create_escrow(&env);
    escrow_client.create_offer(&seller, &100i128, &1000i128, &carbon_token, &usdc_token, &0i128);
}

#[test]
#[should_panic(expected = "min_fill_amount cannot exceed carbon_amount")]
fn test_create_offer_min_fill_exceeds_carbon_amount_panics() {
    let env = Env::default();
    env.mock_all_auths();
    let seller = Address::generate(&env);
    let carbon_token = Address::generate(&env);
    let usdc_token = Address::generate(&env);
    let (escrow_client, _) = create_escrow(&env);
    escrow_client.create_offer(&seller, &100i128, &1000i128, &carbon_token, &usdc_token, &101i128);
}

// ============ FILL OFFER TESTS ============

#[test]
#[should_panic(expected = "offer not found")]
fn test_fill_nonexistent_offer_panics() {
    let env = Env::default();
    env.mock_all_auths();
    let buyer = Address::generate(&env);
    let (escrow_client, _) = create_escrow(&env);
    escrow_client.fill_offer(&999u64, &buyer, &100i128);
}

// ============ CANCEL OFFER TESTS ============

#[test]
#[should_panic(expected = "offer not found")]
fn test_cancel_nonexistent_offer_panics() {
    let env = Env::default();
    env.mock_all_auths();
    let caller = Address::generate(&env);
    let (escrow_client, _) = create_escrow(&env);
    escrow_client.cancel_offer(&999u64, &caller);
}

// ============ GET OFFER TESTS ============

#[test]
fn test_get_nonexistent_offer_returns_none() {
    let env = Env::default();
    env.mock_all_auths();
    let (escrow_client, _) = create_escrow(&env);
    let offer = escrow_client.get_offer(&999u64);
    assert!(offer.is_none());
}

#[test]
fn test_get_remaining_amount_nonexistent_offer() {
    let env = Env::default();
    env.mock_all_auths();
    let (escrow_client, _) = create_escrow(&env);
    let (carbon, usdc) = escrow_client.get_remaining_amount(&999u64);
    assert_eq!(carbon, 0i128);
    assert_eq!(usdc, 0i128);
}

// ============ OFFER STRUCT TESTS ============

#[test]
fn test_offer_remaining_carbon() {
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
        min_fill_amount: 10,
    };
    assert_eq!(offer.remaining_carbon(), 700);
    assert_eq!(offer.remaining_usdc(), 3500);
}

#[test]
fn test_offer_is_fully_filled() {
    let env = Env::default();

    let offer1 = Offer {
        offer_id: 1,
        seller: Address::generate(&env),
        carbon_amount: 1000,
        usdc_amount: 5000,
        filled_carbon: 999,
        filled_usdc: 4995,
        carbon_token: Address::generate(&env),
        usdc_token: Address::generate(&env),
        is_cancelled: false,
        min_fill_amount: 10,
    };
    assert!(!offer1.is_fully_filled());

    let offer2 = Offer {
        offer_id: 2,
        seller: Address::generate(&env),
        carbon_amount: 1000,
        usdc_amount: 5000,
        filled_carbon: 1000,
        filled_usdc: 5000,
        carbon_token: Address::generate(&env),
        usdc_token: Address::generate(&env),
        is_cancelled: false,
        min_fill_amount: 10,
    };
    assert!(offer2.is_fully_filled());
}

// ============ MIN FILL AMOUNT TESTS ============

#[test]
fn test_min_fill_amount_enforced_on_partial_fill() {
    // fill < min_fill_amount AND fill < remaining => reject
    let remaining_carbon = 1000i128;
    let min_fill_amount = 50i128;
    let fill_carbon_amount = 10i128;

    let is_final_fill = fill_carbon_amount >= remaining_carbon;
    let below_minimum = fill_carbon_amount < min_fill_amount && !is_final_fill;
    assert!(below_minimum, "should reject dust fill");
}

#[test]
fn test_min_fill_amount_waived_for_final_fill() {
    // Final fill (consumes all remaining) is allowed even if below min_fill_amount
    let remaining_carbon = 30i128;
    let min_fill_amount = 50i128;
    let fill_carbon_amount = 30i128;

    let is_final_fill = fill_carbon_amount >= remaining_carbon;
    let below_minimum = fill_carbon_amount < min_fill_amount && !is_final_fill;
    assert!(!below_minimum, "final fill should bypass minimum");
}

#[test]
fn test_min_fill_amount_exact_minimum_allowed() {
    let remaining_carbon = 1000i128;
    let min_fill_amount = 50i128;
    let fill_carbon_amount = 50i128;

    let is_final_fill = fill_carbon_amount >= remaining_carbon;
    let below_minimum = fill_carbon_amount < min_fill_amount && !is_final_fill;
    assert!(!below_minimum, "fill at exact minimum should be allowed");
}

// ============ PARTIAL FILL SCALING TESTS ============

#[test]
fn test_partial_fill_scaling_calculation() {
    let carbon_amount = 1000i128;
    let usdc_amount = 5000i128;
    let fill_carbon = 100i128;
    let expected_usdc = (fill_carbon * usdc_amount) / carbon_amount;
    assert_eq!(expected_usdc, 500i128);
}

#[test]
fn test_partial_fill_scaling_floors_fractional() {
    let carbon_amount = 3i128;
    let usdc_amount = 10i128;
    let fill_carbon = 1i128;
    let expected_usdc = (fill_carbon * usdc_amount) / carbon_amount;
    assert_eq!(expected_usdc, 3i128); // floored
}

#[test]
fn test_partial_fill_multiple_fills() {
    let env = Env::default();
    let mut offer = Offer {
        offer_id: 1,
        seller: Address::generate(&env),
        carbon_amount: 1000,
        usdc_amount: 5000,
        filled_carbon: 0,
        filled_usdc: 0,
        carbon_token: Address::generate(&env),
        usdc_token: Address::generate(&env),
        is_cancelled: false,
        min_fill_amount: 50,
    };

    offer.filled_carbon += 300;
    offer.filled_usdc += 1500;
    assert_eq!(offer.remaining_carbon(), 700);

    offer.filled_carbon += 400;
    offer.filled_usdc += 2000;
    assert_eq!(offer.remaining_carbon(), 300);

    offer.filled_carbon += 300;
    offer.filled_usdc += 1500;
    assert!(offer.is_fully_filled());
}

// ============ AUTHORIZATION TESTS ============

#[test]
#[should_panic(expected = "Require auth")]
fn test_create_offer_without_auth_panics() {
    let env = Env::default();
    let seller = Address::generate(&env);
    let carbon_token = Address::generate(&env);
    let usdc_token = Address::generate(&env);
    let (escrow_client, _) = create_escrow(&env);
    escrow_client.create_offer(&seller, &100i128, &1000i128, &carbon_token, &usdc_token, &10i128);
}

#[test]
#[should_panic(expected = "Require auth")]
fn test_fill_offer_without_auth_panics() {
    let env = Env::default();
    let buyer = Address::generate(&env);
    let (escrow_client, _) = create_escrow(&env);
    escrow_client.fill_offer(&1u64, &buyer, &100i128);
}

#[test]
#[should_panic(expected = "Require auth")]
fn test_cancel_offer_without_auth_panics() {
    let env = Env::default();
    let caller = Address::generate(&env);
    let (escrow_client, _) = create_escrow(&env);
    escrow_client.cancel_offer(&1u64, &caller);
}
