use loam_sdk::soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// No such Contract has been published
    NoSuchContractPublished = 1,
    /// No such version of the contact has been published
    NoSuchVersion = 2,
    /// Contract already published
    AlreadyPublished = 3,

    /// No such contract deployed
    NoSuchContractDeployed = 4,

    /// Contract already deployed
    AlreadyDeployed = 5,
}
