#![no_std]
use loam_sdk_core_riffs::{owner::Owner, Ownable, Redeployable};

pub mod gen;

//#[loam]
pub struct Contract;

impl Ownable for Contract {
    type Impl = Owner;
}

impl Redeployable for Contract {}
