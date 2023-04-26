#![no_std]
use loam_sdk::soroban_sdk::String;
use loam_sdk_core_riffs::{owner::Owner, Ownable, Redeployable};

pub trait Fetchable {
    fn fetch_hash(contract_name: String, version: Option<Version>) -> Bytes<32> {}

    fn fetch(contract_name: String, version: Option<Version>) {}
}
