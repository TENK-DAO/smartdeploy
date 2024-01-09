#![cfg(test)]
use super::*;
use crate::{error::Error, SorobanContract, SorobanContractClient};
use loam_sdk::soroban_sdk::{
    testutils::{ Address as _, Events },
    Address, Bytes, Env, String, IntoVal,
    vec, symbol_short,
};
extern crate std;

// The contract that will be deployed by the Publisher contract.
mod contract {
    use loam_sdk::soroban_sdk;
    soroban_sdk::contractimport!(file = "../../target/loam/smartdeploy.wasm");
}

fn init() -> (Env, SorobanContractClient<'static>, Address) {
    let env = Env::default();
    let client = SorobanContractClient::new(&env, &env.register_contract(None, SorobanContract));
    let address = Address::generate(&env);
    (env, client, address)
}

pub fn name(env: &Env) -> String {
    String::from_str(env, "publisher")
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

    let other_address = Address::generate(env);
    let res = client
        .try_publish(name, &other_address, &bytes, &None, &None)
        .unwrap_err();

    assert!(matches!(res, Ok(Error::AlreadyPublished)));

    // let res = client.try_deploy(name, &None, &String::from_slice(env, "hello"), &None);

    // let res = client.try_deploy(name, &None, &String::from_slice(env, "hello"), &None);

    // let res = client.try_deploy(name, &None, &String::from_slice(env, "hello"), &None);
    // std::println!("{res:?}");
}

#[test]
fn publish_event() {

    let (env, client, address) = &init();
    env.mock_all_auths();
    
    let name = String::from_str(env, "contract_a");

    let bytes = Bytes::from_slice(env, contract::WASM);
    
    client.publish(&name, address, &bytes, &None, &None);

    let data =  registry::wasm::PublishEventData {
        published_name: name,
        author: address.clone(),
        wasm: bytes,
        repo: metadata::ContractMetadata::default(),
        kind: version::Update::default(),
    };

    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                client.address.clone(),
                (symbol_short!("publish"),).into_val(env),
                data.into_val(env)
            ),
        ]
    );
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
