#![no_std]

use loam_sdk::soroban_sdk::{
    self, contract, contractimpl, symbol_short, Address, BytesN, Env, Symbol,
};

#[contract]
pub struct ContractA;

#[contractimpl]
impl ContractA {
    pub fn add(x: u32, y: u32) -> u32 {
        x.checked_add(y).expect("no overflow")
    }
}


smartdeploy_sdk::core_riff!();
