#![no_std]

use loam_sdk::soroban_sdk::{
    self, contract, contractimpl, symbol_short, Address, BytesN, Env, Symbol,
};

#[contract]
pub struct ContractA;

#[contractimpl]
impl ContractA {
    pub fn add(x: u32, y: u32) -> u32 {
        x.checked_add(y).expect("no overflow")
    }

    pub fn owner_get(env: Env) -> Option<Address> {
        env.storage().instance().get(&Self::owner_key())
    }

    pub fn owner_set(env: Env, new_owner: Address) {
        Self::owner_get(env.clone()).as_ref().map(Address::require_auth);
        env.storage().instance().set(&Self::owner_key(), &new_owner);
    }

    pub fn redeploy(env: Env, wasm_hash: BytesN<32>) {
        Self::owner_get(env.clone()).as_ref().map(Address::require_auth);
        env.deployer().update_current_contract_wasm(wasm_hash);
    }
    fn owner_key() -> Symbol {
        symbol_short!("owner")
    }
}
