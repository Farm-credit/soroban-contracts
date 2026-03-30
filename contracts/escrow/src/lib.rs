#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env,
};

mod storage {
    use soroban_sdk::{Env, Address};

    const INSTANCE_BUMP_AMOUNT: u32 = 16777215;
    const INSTANCE_LIFETIME_THRESHOLD: u32 = 10368000;

    const OFFERS_PREFIX: &str = "offers";
    const OFFER_COUNT_KEY: &str = "offer_count";
    const INITIALIZED_KEY: &str = "initialized";
    const PAUSED_KEY: &str = "paused";
    const SUPER_ADMIN_KEY: &str = "super_admin";

    pub fn extend_ttl(env: &Env) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    }

    pub fn is_initialized(env: &Env) -> bool {
        env.storage()
            .instance()
            .get::<_, bool>(&INITIALIZED_KEY)
            .unwrap_or(false)
    }

    pub fn set_initialized(env: &Env) {
        env.storage().instance().set(&INITIALIZED_KEY, &true);
    }

    pub fn read_offer_count(env: &Env) -> u64 {
        env.storage()
            .instance()
            .get::<_, u64>(&OFFER_COUNT_KEY)
            .unwrap_or(0)
    }

    pub fn write_offer_count(env: &Env, count: u64) {
        env.storage().instance().set(&OFFER_COUNT_KEY, &count);
    }

    pub fn store_offer(env: &Env, offer_id: u64, offer: &super::Offer) {
        let key = (OFFERS_PREFIX.as_bytes(), offer_id);
        env.storage().instance().set(&key, offer);
    }

    pub fn get_offer(env: &Env, offer_id: u64) -> Option<super::Offer> {
        let key = (OFFERS_PREFIX.as_bytes(), offer_id);
        env.storage().instance().get(&key)
    }

    pub fn remove_offer(env: &Env, offer_id: u64) {
        let key = (OFFERS_PREFIX.as_bytes(), offer_id);
        env.storage().instance().remove(&key);
    }

    pub fn is_paused(env: &Env) -> bool {
        env.storage()
            .instance()
            .get::<_, bool>(&PAUSED_KEY)
            .unwrap_or(false)
    }

    pub fn set_paused(env: &Env, paused: bool) {
        env.storage().instance().set(&PAUSED_KEY, &paused);
    }

    pub fn write_super_admin(env: &Env, admin: &Address) {
        env.storage().instance().set(&SUPER_ADMIN_KEY, admin);
    }

    pub fn read_super_admin(env: &Env) -> Address {
        env.storage()
            .instance()
            .get(&SUPER_ADMIN_KEY)
            .expect("super admin not set")
    }
}

#[derive(Clone)]
#[contracttype]
pub struct Offer {
    pub offer_id: u64,
    pub seller: Address,
    pub carbon_amount: i128,
    pub usdc_amount: i128,
    pub filled_carbon: i128,
    pub filled_usdc: i128,
    pub carbon_token: Address,
    pub usdc_token: Address,
    pub is_cancelled: bool,
    /// The ledger number after which this offer is considered expired.
    pub expiration_ledger: u32,
}

impl Offer {
    pub fn remaining_carbon(&self) -> i128 {
        self.carbon_amount - self.filled_carbon
    }

    pub fn remaining_usdc(&self) -> i128 {
        self.usdc_amount - self.filled_usdc
    }

    pub fn is_fully_filled(&self) -> bool {
        self.filled_carbon >= self.carbon_amount
    }

    pub fn is_expired(&self, current_ledger: u32) -> bool {
        current_ledger > self.expiration_ledger
    }
}

fn require_not_paused(env: &Env) {
    if storage::is_paused(env) {
        panic!("contract is paused");
    }
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    /// Initialize the escrow contract. `super_admin` is the only address
    /// that can pause/unpause the contract.
    pub fn initialize(env: Env, super_admin: Address) {
        storage::extend_ttl(&env);
        if storage::is_initialized(&env) {
            panic!("escrow already initialized");
        }
        storage::set_initialized(&env);
        storage::write_super_admin(&env, &super_admin);
        storage::write_offer_count(&env, 0);
    }

    /// Pause all state-mutating operations. SuperAdmin only.
    pub fn admin_pause(env: Env, admin: Address) {
        admin.require_auth();
        let super_admin = storage::read_super_admin(&env);
        if admin != super_admin {
            panic!("only super admin can pause");
        }
        storage::extend_ttl(&env);
        storage::set_paused(&env, true);
        env.events().publish(("paused",), admin);
    }

    /// Unpause the contract. SuperAdmin only.
    pub fn admin_unpause(env: Env, admin: Address) {
        admin.require_auth();
        let super_admin = storage::read_super_admin(&env);
        if admin != super_admin {
            panic!("only super admin can unpause");
        }
        storage::extend_ttl(&env);
        storage::set_paused(&env, false);
        env.events().publish(("unpaused",), admin);
    }

    /// Returns whether the contract is currently paused.
    pub fn paused(env: Env) -> bool {
        storage::is_paused(&env)
    }

    /// Create a new offer - seller deposits Carbon tokens into escrow.
    /// `expiration_ledger` sets the ledger number after which the offer expires.
    /// Returns the offer_id.
    pub fn create_offer(
        env: Env,
        seller: Address,
        carbon_amount: i128,
        usdc_amount: i128,
        carbon_token: Address,
        usdc_token: Address,
        expiration_ledger: u32,
    ) -> u64 {
        seller.require_auth();
        require_not_paused(&env);

        if carbon_amount <= 0 || usdc_amount <= 0 {
            panic!("amounts must be positive");
        }

        if expiration_ledger <= env.ledger().sequence() {
            panic!("expiration_ledger must be in the future");
        }

        storage::extend_ttl(&env);

        let offer_id = storage::read_offer_count(&env) + 1;
        storage::write_offer_count(&env, offer_id);

        let offer = Offer {
            offer_id,
            seller: seller.clone(),
            carbon_amount,
            usdc_amount,
            filled_carbon: 0,
            filled_usdc: 0,
            carbon_token: carbon_token.clone(),
            usdc_token: usdc_token.clone(),
            is_cancelled: false,
            expiration_ledger,
        };

        storage::store_offer(&env, offer_id, &offer);

        // Transfer Carbon tokens from seller to escrow
        let carbon_client = soroban_sdk::token::Client::new(&env, &carbon_token);
        carbon_client.transfer(&seller, &env.current_contract_address(), &carbon_amount);

        env.events().publish(
            ("offer_created",),
            (offer_id, seller.clone(), carbon_amount, usdc_amount),
        );

        offer_id
    }

    /// Fill an offer - buyer pays USDC and receives Carbon tokens.
    /// Supports partial fills. Rejects fills on expired offers.
    pub fn fill_offer(env: Env, offer_id: u64, buyer: Address, fill_carbon_amount: i128) {
        buyer.require_auth();
        require_not_paused(&env);

        if fill_carbon_amount <= 0 {
            panic!("fill amount must be positive");
        }

        storage::extend_ttl(&env);

        let mut offer = storage::get_offer(&env, offer_id).expect("offer not found");

        if offer.is_cancelled {
            panic!("offer is cancelled");
        }

        if offer.is_expired(env.ledger().sequence()) {
            panic!("offer is expired");
        }

        let remaining_carbon = offer.remaining_carbon();
        if fill_carbon_amount > remaining_carbon {
            panic!("fill amount exceeds remaining offer amount");
        }

        // Calculate proportional USDC amount
        let fill_usdc_amount = (fill_carbon_amount * offer.usdc_amount) / offer.carbon_amount;

        // Transfer USDC from buyer to escrow
        let usdc_client = soroban_sdk::token::Client::new(&env, &offer.usdc_token);
        usdc_client.transfer(&buyer, &env.current_contract_address(), &fill_usdc_amount);

        // Transfer Carbon tokens from escrow to buyer
        let carbon_client = soroban_sdk::token::Client::new(&env, &offer.carbon_token);
        carbon_client.transfer(&env.current_contract_address(), &buyer, &fill_carbon_amount);

        // Transfer USDC from escrow to seller
        usdc_client.transfer(&env.current_contract_address(), &offer.seller, &fill_usdc_amount);

        // Update offer with filled amounts
        offer.filled_carbon += fill_carbon_amount;
        offer.filled_usdc += fill_usdc_amount;

        if offer.is_fully_filled() {
            storage::remove_offer(&env, offer_id);
        } else {
            storage::store_offer(&env, offer_id, &offer);
        }

        env.events().publish(
            ("offer_filled",),
            (offer_id, buyer.clone(), fill_carbon_amount, fill_usdc_amount),
        );
    }

    /// Cancel an offer - only the seller can cancel.
    /// Returns remaining carbon tokens to seller.
    pub fn cancel_offer(env: Env, offer_id: u64, caller: Address) {
        caller.require_auth();
        require_not_paused(&env);

        storage::extend_ttl(&env);

        let mut offer = storage::get_offer(&env, offer_id).expect("offer not found");

        if caller != offer.seller {
            panic!("only the seller can cancel this offer");
        }

        if offer.is_cancelled {
            panic!("offer already cancelled");
        }

        let remaining_carbon = offer.remaining_carbon();
        if remaining_carbon > 0 {
            let carbon_client = soroban_sdk::token::Client::new(&env, &offer.carbon_token);
            carbon_client.transfer(&env.current_contract_address(), &offer.seller, &remaining_carbon);
        }

        offer.is_cancelled = true;
        storage::store_offer(&env, offer_id, &offer);

        env.events().publish(
            ("offer_cancelled",),
            (offer_id, offer.seller.clone(), remaining_carbon),
        );
    }

    /// Reclaim tokens from an expired offer.
    /// Anyone can call this, but tokens always return to the seller.
    /// Cleans up ledger storage for the expired offer.
    pub fn reclaim_expired(env: Env, offer_id: u64) {
        require_not_paused(&env);
        storage::extend_ttl(&env);

        let offer = storage::get_offer(&env, offer_id).expect("offer not found");

        if offer.is_cancelled {
            panic!("offer is already cancelled");
        }

        if !offer.is_expired(env.ledger().sequence()) {
            panic!("offer has not expired yet");
        }

        let remaining_carbon = offer.remaining_carbon();
        if remaining_carbon > 0 {
            let carbon_client = soroban_sdk::token::Client::new(&env, &offer.carbon_token);
            carbon_client.transfer(&env.current_contract_address(), &offer.seller, &remaining_carbon);
        }

        // Remove the offer to reclaim ledger storage
        storage::remove_offer(&env, offer_id);

        env.events().publish(
            ("offer_reclaimed",),
            (offer_id, offer.seller.clone(), remaining_carbon),
        );
    }

    /// Extend the expiration of an offer. Only the seller can extend.
    /// `new_expiration_ledger` must be greater than the current expiration.
    pub fn extend_offer_expiration(env: Env, offer_id: u64, seller: Address, new_expiration_ledger: u32) {
        seller.require_auth();
        require_not_paused(&env);

        storage::extend_ttl(&env);

        let mut offer = storage::get_offer(&env, offer_id).expect("offer not found");

        if seller != offer.seller {
            panic!("only the seller can extend this offer");
        }

        if offer.is_cancelled {
            panic!("offer is cancelled");
        }

        if new_expiration_ledger <= offer.expiration_ledger {
            panic!("new expiration must be later than current expiration");
        }

        offer.expiration_ledger = new_expiration_ledger;
        storage::store_offer(&env, offer_id, &offer);

        env.events().publish(
            ("offer_expiration_extended",),
            (offer_id, seller.clone(), new_expiration_ledger),
        );
    }

    /// Get offer details
    pub fn get_offer(env: Env, offer_id: u64) -> Option<Offer> {
        storage::extend_ttl(&env);
        storage::get_offer(&env, offer_id)
    }

    /// Get remaining amount for an offer
    pub fn get_remaining_amount(env: Env, offer_id: u64) -> (i128, i128) {
        storage::extend_ttl(&env);
        if let Some(offer) = storage::get_offer(&env, offer_id) {
            (offer.remaining_carbon(), offer.remaining_usdc())
        } else {
            (0, 0)
        }
    }
}

mod test;
