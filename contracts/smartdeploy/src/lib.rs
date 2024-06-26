#![no_std]
use loam_sdk::{soroban_contract, soroban_sdk};
use loam_sdk_core_riff::{owner::Owner, CoreRiff};
use registry::{
    contract::ContractRegistry, wasm::WasmRegistry, Deployable, DevDeployable, Publishable, Claimable,
};

pub mod error;
pub mod events;
pub mod metadata;
pub mod registry;
pub mod util;
pub mod version;

use error::Error;
use version::Version;

pub struct Contract;

impl Publishable for Contract {
    type Impl = WasmRegistry;
}

impl Deployable for Contract {
    type Impl = ContractRegistry;
}

impl Claimable for Contract {
    type Impl = ContractRegistry;
}

impl DevDeployable for Contract {
    type Impl = ContractRegistry;
}

impl CoreRiff for Contract {
    type Impl = Owner;
}

soroban_contract!();

#[cfg(test)]
mod test;
