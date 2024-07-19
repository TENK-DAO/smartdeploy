#![no_std]


use loam_sdk::derive_contract;
use loam_subcontract_core::{Admin, Core};

pub mod counter;
pub use counter::{Riff, Impl};

#[derive_contract(Core(Admin), Riff(Impl))]
struct Contract;


mod test;
