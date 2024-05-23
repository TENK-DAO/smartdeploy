#![no_std]
use loam_sdk::soroban_contract;

pub mod counter;
pub mod error;

use counter::Riff;
use loam_subcontract_core::Core;

struct Contract;

impl counter::Riff for Contract {
    type Impl = counter::Impl;
}

impl Core for Contract {
    type Impl = loam_subcontract_core::Admin;
}

soroban_contract!();

mod test;
