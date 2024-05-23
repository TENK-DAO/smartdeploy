#![cfg(test)]
#![allow(clippy::similar_names)]

use crate::{ContractA, ContractAClient};
use loam_sdk::soroban_sdk::Env;

#[test]
fn test() {
    let env = Env::default();

    // // Register contract A using the imported WASM.
    let contract_a_id = env.register_contract(None, ContractA);

    // // // Register contract B defined in this crate.
    // let contract_b_id = env.register_contract(None, ContractB);

    // Create a client for calling contract B.
    let client = ContractAClient::new(&env, &contract_a_id);

    // Invoke contract B via its client. Contract B will invoke contract A.
    let sum = client.add(&5, &7);
    assert_eq!(sum, 12);
}
