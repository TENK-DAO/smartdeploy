#![no_std]
use loam_sdk::{derive_contract, soroban_sdk};
use loam_subcontract_core::{Admin, Core};
use registry::{
    contract::ContractRegistry, wasm::WasmRegistry, Claimable, Deployable, DevDeployable,
    Publishable,
};

pub mod error;
pub mod events;
pub mod metadata;
pub mod registry;
pub mod util;
pub mod version;

use error::Error;
use version::Version;

#[derive_contract(
    Core(Admin),
    Claimable(ContractRegistry),
    Publishable(WasmRegistry),
    Deployable(ContractRegistry),
    DevDeployable(ContractRegistry)
)]
pub struct Contract;


#[cfg(test)]
mod test;
