#![cfg(test)]
#![allow(clippy::similar_names)]

use crate::{contract_a, contract_b, ContractC, ContractCClient};
use loam_sdk::soroban_sdk::Env;

#[test]
fn test() {
    let env = Env::default();

    // Register contract A using the imported WASM.
    let contract_a_id = env.register_contract_wasm(None, contract_a::WASM);

    // Register contract B defined in this crate.
    let contract_b_id = env.register_contract_wasm(None, contract_b::WASM);

    let contract_c = env.register_contract(None, ContractC);
    let client = ContractCClient::new(&env, &contract_c);

    // Invoke contract B via its client. Contract B will invoke contract A.
    let sum = client.add_with_a_and_b(&contract_a_id, &contract_b_id, &5, &7);
    assert_eq!(sum, 12);
}
