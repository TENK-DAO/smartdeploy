#![no_std]
use loam_sdk_core_riffs::{owner::Owner, Ownable, Redeployable};
use registry::{Binary, ContractRegistry, Deployable};

extern crate alloc;

pub mod error;
pub mod gen;
pub mod metadata;
pub mod registry;
pub mod util;
pub mod version;

pub struct Contract;

impl Binary for Contract {
    type Impl = ContractRegistry;
}

impl Deployable for Contract {
    type Impl = ContractRegistry;
}

impl Ownable for Contract {
    type Impl = Owner;
}

impl Redeployable for Contract {}

#[cfg(test)]
mod test;
