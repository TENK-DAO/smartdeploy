#![no_std]
use loam_sdk::soroban_contract;
use loam_sdk::soroban_sdk;

pub mod counter;
pub mod error;

use counter::Riff;
use loam_sdk_core_riff::CoreRiff;

struct Contract;

impl counter::Riff for Contract {
    type Impl = counter::Impl;
}

impl CoreRiff for Contract {
    type Impl = loam_sdk_core_riff::owner::Owner;
}

soroban_contract!();

mod test;
