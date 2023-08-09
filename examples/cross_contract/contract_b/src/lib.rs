#![no_std]

use loam_sdk::soroban_sdk::{self, contract, contractimpl, Address, Env};

mod contract_a {
    use loam_sdk::soroban_sdk;
    loam_sdk::soroban_sdk::contractimport!(
        file = "../../../target/loam/soroban_cross_contract_a_contract.wasm",
    );
}

#[contract]
pub struct ContractB;

#[contractimpl]
impl ContractB {
    pub fn add_with(env: Env, contract_id: Address, x: u32, y: u32) -> u32 {
        let client = contract_a::Client::new(&env, &contract_id);
        client.add(&x, &y)
    }
}

mod test;
