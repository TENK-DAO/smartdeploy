#![cfg(test)]

use crate::{Publisher, PublisherClient};
use soroban_sdk::{
    testutils::{Address as _, BytesN as _},
    Address, BytesN, Env, String,
};
extern crate std;

// The contract that will be deployed by the Publisher contract.
mod contract {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/smart_deploy.wasm"
    );
}

fn init() -> (Env, PublisherClient, Address) {
    let env = Env::default();
    let client = PublisherClient::new(&env, &env.register_contract(None, Publisher));
    let address = Address::random(&env);
    (env, client, address)
}

pub fn name(env: &Env) -> String {
    String::from_slice(env, "publisher")
}

#[test]
fn handle_error_cases() {
    let (env, client, address) = &init();

    let name = &name(env);
    let res = client.try_fetch(name, &None).unwrap_err();
    assert!(matches!(res, Ok(crate::Error::NoSuchContract)));
    client.register_name(address, name);
    let wasm_hash = env.install_contract_wasm(contract::WASM);

    let res = client.try_fetch(name, &None).unwrap_err();
    assert!(matches!(res, Ok(crate::Error::NoSuchVersion)));

    client.publish_binary(name, &wasm_hash, &None, &None);
    let res = client.try_fetch(name, &None).unwrap().unwrap();
    assert_eq!(res, wasm_hash);

    let other_address = Address::random(env);
    let res = client.try_register_name(&other_address, name).unwrap_err();
    assert!(matches!(res, Ok(crate::Error::AlreadyRegistered)));

    let res = client.get_num_deploys(name, &None);
    std::println!("num {res:?}");

    let res = client.try_deploy(name, &None, &String::from_slice(env, "hello"), &None);
    std::println!("{res:?}");

    let res = client.get_num_deploys(name, &None);
    std::println!("num {res:?}");

    let res = client.try_deploy(name, &None, &String::from_slice(env, "hello"), &None);
    std::println!("{res:?}");

    let res = client.get_num_deploys(name, &None);
    std::println!("num {res:?}");

    let res = client.try_deploy(name, &None, &String::from_slice(env, "hello"), &None);
    std::println!("{res:?}");
}

#[test]
fn returns_most_recent_version() {
    let (env, client, address) = &init();
    let name = &name(env);
    client.register_name(address, name);
    let wasm_hash = env.install_contract_wasm(contract::WASM);

    client.publish_binary(name, &wasm_hash, &None, &None);

    let second_hash: BytesN<32> = BytesN::random(env);
    client.publish_binary(name, &second_hash, &None, &None);
    let res = client.fetch(name, &None);
    assert_eq!(res, second_hash);

    let third_hash: BytesN<32> = BytesN::random(env);
    client.publish_binary(name, &third_hash, &None, &None);
    let res = client.fetch(name, &None);
    assert_eq!(res, third_hash);

    let third_hash: BytesN<32> = BytesN::random(env);
    client.publish_binary(name, &third_hash, &None, &None);
    let res = client.fetch(name, &None);
    assert_eq!(res, third_hash);
}
