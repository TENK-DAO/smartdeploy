#![no_std]

use loam_sdk::soroban_sdk::{self, contract, contractimpl, Address, Env};

loam_sdk::import_contract!(contract_a);

#[contract]
pub struct ContractB;

#[contractimpl]
impl ContractB {
    pub fn add_with(env: Env, contract_id: Address, x: u32, y: u32) -> u32 {
        let client = contract_a::Client::new(&env, &contract_id);
        client.add(&x, &y)
    }

    pub fn owner_get(env: soroban_sdk::Env) -> Option<soroban_sdk::Address> {
        env.storage().instance().get(&Self::owner_key())
    }

    pub fn owner_set(env: soroban_sdk::Env, new_owner: soroban_sdk::Address) {
        Self::owner_get(env.clone()).as_ref().map(soroban_sdk::Address::require_auth);
        env.storage().instance().set(&Self::owner_key(), &new_owner);
    }

    pub fn redeploy(env: soroban_sdk::Env, wasm_hash: soroban_sdk::BytesN<32>) {
        Self::owner_get(env.clone()).as_ref().map(Address::require_auth);
        env.deployer().update_current_contract_wasm(wasm_hash);
    }
    fn owner_key() -> soroban_sdk::Symbol {
        soroban_sdk::symbol_short!("owner")
    }
}

mod test;
