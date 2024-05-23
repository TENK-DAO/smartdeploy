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
}

smartdeploy_sdk::core!();

mod test;
