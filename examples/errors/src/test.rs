#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Logs, Env};

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, SorobanContract);
    let client = SorobanContractClient::new(&env, &contract_id);

    assert_eq!(client.try_increment(), Ok(Ok(1)));
    assert_eq!(client.try_increment(), Ok(Ok(2)));
    assert_eq!(client.try_increment(), Ok(Ok(3)));
    assert_eq!(client.try_increment(), Ok(Ok(4)));
    assert_eq!(client.try_increment(), Ok(Ok(5)));
    assert_eq!(client.try_increment(), Err(Ok(error::Error::LimitReached)));

    std::println!("{}", env.logs().all().join("\n"));
}
