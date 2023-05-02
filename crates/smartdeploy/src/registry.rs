use loam_sdk::{
    riff,
    soroban_sdk::{
        self, Lazy,
    },
};

use crate::{
    error::Error,
    version::{self, Version},
};

pub mod contract;
pub mod wasm;

#[riff]

pub trait IsBinary {
    /// Fetch the hash from the registry
    fn fetch_hash(
        &self,
        contract_name: soroban_sdk::String,
        version: Option<Version>,
    ) -> Result<soroban_sdk::BytesN<32>, Error> {
        Ok(self.fetch(contract_name, version)?.hash)
    }

    /// Most recent version of the published contract
    fn current_version(&self, contract_name: soroban_sdk::String) -> Result<Version, Error>;

    /// Fetch detailts of the published binary
    fn fetch(
        &self,
        contract_name: soroban_sdk::String,
        version: Option<Version>,
    ) -> Result<crate::metadata::PublishedWasm, Error>;

    /// Publish a binary. If contract had been previously published only previous author can publish again
    fn publish(
        &mut self,
        contract_name: soroban_sdk::String,
        author: soroban_sdk::Address,
        hash: soroban_sdk::BytesN<32>,
        repo: Option<soroban_sdk::String>,
        kind: Option<version::Update>,
    ) -> Result<(), Error>;
}

#[riff]
pub trait IsDeployable {
    /// Deploy a contract. A contract can only be deployed once
    fn deploy(
        &mut self,
        contract_name: soroban_sdk::String,
        version: Option<Version>,
        deployed_name: soroban_sdk::String,
        owner: soroban_sdk::Address,
        salt: Option<soroban_sdk::BytesN<32>>,
    ) -> Result<soroban_sdk::BytesN<32>, Error>;

    /// Fetch contract id
    fn fetch_contract_id(
        &self,
        deployed_name: soroban_sdk::String,
    ) -> Result<soroban_sdk::BytesN<32>, Error>;
}
