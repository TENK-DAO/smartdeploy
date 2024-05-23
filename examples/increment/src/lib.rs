#![no_std]

use loam_sdk::soroban_contract;
use loam_subcontract_core::{Admin, Core};

pub mod counter;
pub use counter::Riff;

struct Contract;

impl Riff for Contract {
    type Impl = counter::Impl;
}

impl Core for Contract {
    type Impl = Admin;
}

soroban_contract!();

mod test;
