#![no_std]

use loam_sdk::{soroban_contract, soroban_sdk};
use loam_sdk_core_riff::{owner::Owner, CoreRiff};

pub mod counter;
pub use counter::Riff;

struct Contract;

impl Riff for Contract {
    type Impl = counter::Impl;
}

impl CoreRiff for Contract {
    type Impl = Owner;
}

soroban_contract!();

mod test;
