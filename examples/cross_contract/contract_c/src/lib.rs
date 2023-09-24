#![no_std]

use loam_sdk::soroban_sdk::{self, contract, contractimpl, Address, Env};

smartdeploy_sdk::import_contract!(contract_a);

#[contract]
pub struct ContractB;

#[contractimpl]
impl ContractB {
    pub fn add_with(env: Env, contract_id: Address, x: u32, y: u32) -> u32 {
        let client = contract_a::new(&env);
        let client_2 = contract_a::Client::new(&env, &contract_id);
        (client.add(&x, &y) + client_2.add(&x, &y)) >> 2
    }
}

smartdeploy_sdk::core_riff!();

mod test;
