#![no_std]

use loam_sdk::derive_contract;

pub mod counter;

use counter::{Impl, Riff};
use loam_subcontract_core::{Admin, Core};

#[derive_contract(Core(Admin), Riff(Impl))]
struct Contract;

mod test;
