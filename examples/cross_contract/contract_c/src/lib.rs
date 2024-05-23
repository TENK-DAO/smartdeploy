#![no_std]
use loam_sdk::soroban_sdk::{self, contract, contractimpl, Address, Env};

loam_sdk::import_contract!(contract_a);
loam_sdk::import_contract!(contract_b);

#[contract]
pub struct ContractC;

#[contractimpl]
impl ContractC {
    #[allow(clippy::similar_names)]
    pub fn add_with_a_and_b(
        env: Env,
        contract_a_addr: Address,
        contract_b_addr: Address,
        x: u32,
        y: u32,
    ) -> u32 {
        let client = contract_a::Client::new(&env, &contract_a_addr);
        let client_2 = contract_b::Client::new(&env, &contract_b_addr);
        (client.add(&x, &y) + client_2.add_with(&contract_a_addr, &x, &y)) >> 1
    }
}

smartdeploy_sdk::core!();

mod test;
