#![cfg(test)]
use crate::{SorobanContract, SorobanContractClient, error::Error};
use loam_sdk::soroban_sdk::{testutils::Address as _, Address, Env, String, Bytes};
extern crate std;

// The contract that will be deployed by the Publisher contract.
mod contract {
    use loam_sdk::soroban_sdk;
    soroban_sdk::contractimport!(
        file = "../../target/loam/smartdeploy.wasm"
    );
}

fn init() -> (Env, SorobanContractClient<'static>, Address) {
    let env = Env::default();
    let client = SorobanContractClient::new(&env, &env.register_contract(None, SorobanContract));
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

    assert!(matches!(res, Ok(Error::NoSuchContractPublished)));
    let wasm_hash = env.deployer().upload_contract_wasm(contract::WASM);

    let res = client.try_fetch(name, &None).unwrap_err();
    assert!(matches!(res, Ok(Error::NoSuchContractPublished)));
    let bytes = Bytes::from_slice(env, contract::WASM);
    client.publish(name, address, &bytes, &None, &None);
    let res = client.try_fetch(name, &None).unwrap().unwrap();
    assert_eq!(res.hash, wasm_hash);

    let other_address = Address::random(env);
    let res = client.try_publish(name, &other_address, &bytes, &None, &None).unwrap_err();

    assert!(matches!(res, Ok(Error::AlreadyPublished)));

    // let res = client.try_deploy(name, &None, &String::from_slice(env, "hello"), &None);

    // let res = client.try_deploy(name, &None, &String::from_slice(env, "hello"), &None);

    // let res = client.try_deploy(name, &None, &String::from_slice(env, "hello"), &None);
    // std::println!("{res:?}");
}

// #[test]
// fn returns_most_recent_version() {
//     let (env, client, address) = &init();
//     let name = &name(env);
//     // client.register_name(address, name);
//     let wasm_hash = env.install_contract_wasm(contract::WASM);

//     client.publish(name, &wasm_hash, &None, &None);
//     let fetched_hash = client.fetch(name, &None);
//     assert_eq!(fetched_hash, wasm_hash);
//     let second_hash: BytesN<32> = BytesN::random(env);
//     client.publish(name, &second_hash, &None, &None);
//     let res = client.fetch(name, &None);
//     assert_eq!(res, second_hash);

//     let third_hash: BytesN<32> = BytesN::random(env);
//     client.publish(name, &third_hash, &None, &None);
//     let res = client.fetch(name, &None);
//     assert_eq!(res, third_hash);

//     let third_hash: BytesN<32> = BytesN::random(env);
//     client.publish(name, &third_hash, &None, &None);
//     let res = client.fetch(name, &None);
//     assert_eq!(res, third_hash);
// }
