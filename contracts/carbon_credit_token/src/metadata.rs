use soroban_sdk::{Env, String};

use crate::storage::DataKey;

pub fn read_name(e: &Env) -> String {
    let key = DataKey::Name;
    e.storage().instance().get(&key).unwrap()
}

pub fn read_symbol(e: &Env) -> String {
    let key = DataKey::Symbol;
    e.storage().instance().get(&key).unwrap()
}

pub fn read_decimals(e: &Env) -> u32 {
    let key = DataKey::Decimals;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_metadata(e: &Env, name: String, symbol: String, decimals: u32) {
    let key_name = DataKey::Name;
    let key_symbol = DataKey::Symbol;
    let key_decimals = DataKey::Decimals;

    e.storage().instance().set(&key_name, &name);
    e.storage().instance().set(&key_symbol, &symbol);
    e.storage().instance().set(&key_decimals, &decimals);
}
