#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec};

mod storage {
    use soroban_sdk::{Address, Env, Vec};

    const INSTANCE_BUMP_AMOUNT: u32 = 16777215;
    const INSTANCE_LIFETIME_THRESHOLD: u32 = 10368000;

    const OFFERS_PREFIX: &str = "offers";
    const OFFER_COUNT_KEY: &str = "offer_count";
    const INITIALIZED_KEY: &str = "initialized";

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

    pub fn scan_offer_ids(env: &Env, start: u64, limit: u64) -> Vec<u64> {
        let mut ids = Vec::new(env);
        if limit == 0 {
            return ids;
        }

        let mut offer_id = if start == 0 { 1 } else { start };
        let max_offer_id = read_offer_count(env);
        let max_items = if limit > u64::from(u32::MAX) {
            u32::MAX
        } else {
            limit as u32
        };

        while offer_id <= max_offer_id {
            if get_offer(env, offer_id).is_some() {
                ids.push_back(offer_id);
                if ids.len() >= max_items {
                    break;
                }
            }

            if offer_id == u64::MAX {
                break;
            }
            offer_id += 1;
        }

        ids
    }

    pub fn scan_offer_ids_by_seller(env: &Env, seller: &Address) -> Vec<u64> {
        let mut ids = Vec::new(env);
        let max_offer_id = read_offer_count(env);
        let mut offer_id = 1u64;

        while offer_id <= max_offer_id {
            if let Some(offer) = get_offer(env, offer_id) {
                if offer.seller == *seller {
                    ids.push_back(offer.offer_id);
                }
            }

            if offer_id == u64::MAX {
                break;
            }
            offer_id += 1;
        }

        ids
    }
}

mod events {
    use soroban_sdk::{contracttype, Address};

    #[derive(Clone)]
    #[contracttype]
    pub struct OfferCreatedEvent {
        pub offer_id: u64,
        pub seller: Address,
        pub carbon_amount: i128,
        pub usdc_amount: i128,
    }

    #[derive(Clone)]
    #[contracttype]
    pub struct OfferFilledEvent {
        pub offer_id: u64,
        pub buyer: Address,
        pub filled_carbon: i128,
        pub filled_usdc: i128,
    }

    #[derive(Clone)]
    #[contracttype]
    pub struct OfferCancelledEvent {
        pub offer_id: u64,
        pub seller: Address,
        pub remaining_carbon: i128,
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

    pub fn is_active(&self) -> bool {
        !self.is_cancelled && !self.is_fully_filled()
    }
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    /// Initialize the escrow contract.
    pub fn initialize(env: Env) {
        storage::extend_ttl(&env);
        if storage::is_initialized(&env) {
            panic!("escrow already initialized");
        }
        storage::set_initialized(&env);
        storage::write_offer_count(&env, 0);
    }

    /// Create a new offer - seller deposits Carbon tokens into escrow.
    /// Returns the offer_id.
    pub fn create_offer(
        env: Env,
        seller: Address,
        carbon_amount: i128,
        usdc_amount: i128,
        carbon_token: Address,
        usdc_token: Address,
    ) -> u64 {
        seller.require_auth();

        if carbon_amount <= 0 || usdc_amount <= 0 {
            panic!("amounts must be positive");
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
        };

        storage::store_offer(&env, offer_id, &offer);

        // Transfer Carbon tokens from seller to escrow.
        let carbon_client = soroban_sdk::token::Client::new(&env, &carbon_token);
        carbon_client.transfer(&seller, &env.current_contract_address(), &carbon_amount);

        env.events().publish(
            ("offer_created",),
            events::OfferCreatedEvent {
                offer_id,
                seller,
                carbon_amount,
                usdc_amount,
            },
        );

        offer_id
    }

    /// Fill an offer - buyer pays USDC and receives Carbon tokens.
    /// Supports partial fills - amount specifies how much carbon to buy.
    pub fn fill_offer(env: Env, offer_id: u64, buyer: Address, fill_carbon_amount: i128) {
        buyer.require_auth();

        if fill_carbon_amount <= 0 {
            panic!("fill amount must be positive");
        }

        storage::extend_ttl(&env);

        let mut offer = match storage::get_offer(&env, offer_id) {
            Some(offer) => offer,
            None => panic!("offer not found"),
        };

        if offer.is_cancelled {
            panic!("offer is cancelled");
        }

        let remaining_carbon = offer.remaining_carbon();
        if fill_carbon_amount > remaining_carbon {
            panic!("fill amount exceeds remaining offer amount");
        }

        // Proportional USDC amount: (fill_carbon / carbon_amount) * usdc_amount.
        let fill_usdc_amount = (fill_carbon_amount * offer.usdc_amount) / offer.carbon_amount;

        // Transfer USDC from buyer to escrow.
        let usdc_client = soroban_sdk::token::Client::new(&env, &offer.usdc_token);
        usdc_client.transfer(&buyer, &env.current_contract_address(), &fill_usdc_amount);

        // Transfer Carbon from escrow to buyer.
        let carbon_client = soroban_sdk::token::Client::new(&env, &offer.carbon_token);
        carbon_client.transfer(&env.current_contract_address(), &buyer, &fill_carbon_amount);

        // Transfer USDC from escrow to seller.
        usdc_client.transfer(&env.current_contract_address(), &offer.seller, &fill_usdc_amount);

        offer.filled_carbon += fill_carbon_amount;
        offer.filled_usdc += fill_usdc_amount;

        if offer.is_fully_filled() {
            storage::remove_offer(&env, offer_id);
        } else {
            storage::store_offer(&env, offer_id, &offer);
        }

        env.events().publish(
            ("offer_filled",),
            events::OfferFilledEvent {
                offer_id,
                buyer,
                filled_carbon: fill_carbon_amount,
                filled_usdc: fill_usdc_amount,
            },
        );
    }

    /// Cancel an offer - only the seller can cancel.
    /// Returns remaining carbon tokens to seller.
    pub fn cancel_offer(env: Env, offer_id: u64, caller: Address) {
        caller.require_auth();
        storage::extend_ttl(&env);

        let mut offer = match storage::get_offer(&env, offer_id) {
            Some(offer) => offer,
            None => panic!("offer not found"),
        };

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
            events::OfferCancelledEvent {
                offer_id,
                seller: offer.seller,
                remaining_carbon,
            },
        );
    }

    /// Get offer details by ID.
    pub fn get_offer(env: Env, offer_id: u64) -> Option<Offer> {
        storage::extend_ttl(&env);
        storage::get_offer(&env, offer_id)
    }

    /// Get remaining carbon/usdc amount for an offer.
    pub fn get_remaining_amount(env: Env, offer_id: u64) -> (i128, i128) {
        storage::extend_ttl(&env);
        if let Some(offer) = storage::get_offer(&env, offer_id) {
            (offer.remaining_carbon(), offer.remaining_usdc())
        } else {
            (0, 0)
        }
    }

    /// Total number of created offers (monotonic counter).
    pub fn get_offer_count(env: Env) -> u64 {
        storage::extend_ttl(&env);
        storage::read_offer_count(&env)
    }

    /// List active offers using offer-id cursor pagination.
    /// Active = not cancelled and not fully filled.
    pub fn get_active_offers(env: Env, start: u64, limit: u64) -> Vec<Offer> {
        storage::extend_ttl(&env);
        let mut offers = Vec::new(&env);
        if limit == 0 {
            return offers;
        }

        let ids = storage::scan_offer_ids(&env, start, limit);
        for id in ids {
            if let Some(offer) = storage::get_offer(&env, id) {
                if offer.is_active() {
                    offers.push_back(offer);
                }
            }
        }
        offers
    }

    /// List offer IDs created by a seller.
    pub fn get_offers_by_seller(env: Env, seller: Address) -> Vec<u64> {
        storage::extend_ttl(&env);
        storage::scan_offer_ids_by_seller(&env, &seller)
    }

    /// List active offer IDs filtered by carbon/usdc token pair.
    pub fn get_offers_by_token_pair(
        env: Env,
        carbon_token: Address,
        usdc_token: Address,
        start: u64,
        limit: u64,
    ) -> Vec<u64> {
        storage::extend_ttl(&env);
        let mut ids = Vec::new(&env);
        if limit == 0 {
            return ids;
        }

        let max_items = if limit > u64::from(u32::MAX) {
            u32::MAX
        } else {
            limit as u32
        };

        let scan_ids = storage::scan_offer_ids(&env, start, limit);
        for id in scan_ids {
            if let Some(offer) = storage::get_offer(&env, id) {
                if offer.is_active()
                    && offer.carbon_token == carbon_token
                    && offer.usdc_token == usdc_token
                {
                    ids.push_back(offer.offer_id);
                    if ids.len() >= max_items {
                        break;
                    }
                }
            }
        }

        ids
    }
}

mod test;
