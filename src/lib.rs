#![no_std]

use soroban_sdk::{contract, contractimpl, Env, String};

#[contract]
pub struct MyContract;

#[contractimpl]
impl MyContract {
    pub fn save_value(env: Env, key: String, value: String) {
        let storage = env.storage().persistent();
        storage.set(&key, &value);
    }

    pub fn get_value(env: Env, key: String) -> Option<String> {
        let storage = env.storage().persistent();
        storage.get(&key)
    }
}
