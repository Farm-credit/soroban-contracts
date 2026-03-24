#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Bytes, Env};

mod events {
    use soroban_sdk::{contracttype, symbol_short, Address, Bytes, Env};

    #[derive(Clone)]
    #[contracttype]
    pub struct VerificationReport {
        pub report_hash: Bytes,
        pub verifier: Address,
        pub farmer: Address,
        pub amount: i128,
        pub timestamp: u64,
        pub used: bool,
    }

    #[derive(Clone)]
    #[contracttype]
    pub struct VerificationAddedEvent {
        pub report_hash: Bytes,
        pub verifier: Address,
        pub farmer: Address,
        pub amount: i128,
    }

    impl VerificationAddedEvent {
        pub fn publish(self, env: &Env) {
            env.events().publish(
                (symbol_short!("verify"), self.report_hash),
                (self.verifier, self.farmer, self.amount),
            );
        }
    }
}

mod storage {
    use soroban_sdk::{Env, Vec, Address, Bytes};

    const INSTANCE_BUMP_AMOUNT: u32 = 16777215;
    const INSTANCE_LIFETIME_THRESHOLD: u32 = 10368000;

    pub fn extend_ttl(env: &Env) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    }

    const ADMIN_KEY: &str = "admin";
    const VERIFIERS_SET_KEY: &str = "verifiers";

    pub fn read_admin(env: &Env) -> Address {
        env.storage()
            .instance()
            .get::<_, Address>(&ADMIN_KEY)
            .unwrap_or_else(|| panic!("admin not set"))
    }

    pub fn write_admin(env: &Env, admin: &Address) {
        env.storage().instance().set(&ADMIN_KEY, admin);
    }

    pub fn is_verifier(env: &Env, addr: &Address) -> bool {
        let verifiers: Vec<Address> = env
            .storage()
            .instance()
            .get(&VERIFIERS_SET_KEY)
            .unwrap_or_else(|| Vec::new(env));
        
        for i in 0..verifiers.len() {
            if let Some(v) = verifiers.get(i) {
                if v == *addr {
                    return true;
                }
            }
        }
        false
    }

    pub fn add_verifier(env: &Env, verifier: &Address) {
        let mut verifiers: Vec<Address> = env
            .storage()
            .instance()
            .get(&VERIFIERS_SET_KEY)
            .unwrap_or_else(|| Vec::new(env));
        
        verifiers.push_back(verifier.clone());
        env.storage().instance().set(&VERIFIERS_SET_KEY, &verifiers);
    }

    const REPORTS_PREFIX: &str = "report_";

    pub fn store_verification_report(env: &Env, report_hash: &Bytes, report: &super::events::VerificationReport) {
        let key = (REPORTS_PREFIX.as_bytes(), report_hash.clone());
        env.storage().instance().set(&key, report);
    }

    pub fn get_verification_report(env: &Env, report_hash: &Bytes) -> Option<super::events::VerificationReport> {
        let key = (REPORTS_PREFIX.as_bytes(), report_hash.clone());
        env.storage().instance().get::<_, super::events::VerificationReport>(&key)
    }

    pub fn mark_report_used(env: &Env, report_hash: &Bytes) {
        if let Some(mut report) = get_verification_report(env, report_hash) {
            report.used = true;
            store_verification_report(env, report_hash, &report);
        }
    }
}

mod admin {
    use super::storage;
    use soroban_sdk::Env;

    pub fn read_administrator(env: &Env) -> soroban_sdk::Address {
        storage::read_admin(env)
    }

    pub fn write_administrator(env: &Env, admin: &soroban_sdk::Address) {
        storage::write_admin(env, admin);
    }
}

#[contract]
pub struct VerifierRegistry;

#[contractimpl]
impl VerifierRegistry {
    pub fn initialize(env: Env, admin: Address) {
        admin::write_administrator(&env, &admin);
    }

    pub fn add_verifier(env: Env, verifier: Address) {
        let admin = admin::read_administrator(&env);
        admin.require_auth();
        
        storage::extend_ttl(&env);
        storage::add_verifier(&env, &verifier);
    }

    pub fn log_verification(
        env: Env,
        verifier: Address,
        report_hash: Bytes,
        farmer: Address,
        amount: i128,
    ) {
        if amount < 0 {
            panic!("amount must be non-negative");
        }

        // Require auth from the verifier
        verifier.require_auth();

        if !storage::is_verifier(&env, &verifier) {
            panic!("verifier is not registered");
        }

        storage::extend_ttl(&env);

        if storage::get_verification_report(&env, &report_hash).is_some() {
            panic!("report_hash already exists");
        }

        let timestamp = env.ledger().timestamp();

        let report = events::VerificationReport {
            report_hash: report_hash.clone(),
            verifier: verifier.clone(),
            farmer,
            amount,
            timestamp,
            used: false,
        };

        storage::store_verification_report(&env, &report_hash, &report);

        events::VerificationAddedEvent {
            report_hash: report_hash.clone(),
            verifier: verifier.clone(),
            farmer: report.farmer.clone(),
            amount,
        }.publish(&env);
    }

    pub fn verify_report(env: Env, report_hash: Bytes) -> Option<events::VerificationReport> {
        storage::extend_ttl(&env);
        storage::get_verification_report(&env, &report_hash)
    }

    pub fn mark_report_used(env: Env, report_hash: Bytes) {
        storage::extend_ttl(&env);
        storage::mark_report_used(&env, &report_hash);
    }

    pub fn is_report_used(env: Env, report_hash: Bytes) -> bool {
        storage::extend_ttl(&env);
        if let Some(report) = storage::get_verification_report(&env, &report_hash) {
            return report.used;
        }
        false
    }

    pub fn is_verifier(env: Env, addr: Address) -> bool {
        storage::extend_ttl(&env);
        storage::is_verifier(&env, &addr)
    }
}