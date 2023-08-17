use loam_sdk::{
    riff,
    soroban_sdk::{self, Lazy},
};

use crate::{
    error::Error,
    version::{self, Version},
};

pub mod contract;
pub mod wasm;

#[riff]
pub trait IsPublishable {
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
        bytes: soroban_sdk::Bytes,
        repo: Option<soroban_sdk::String>,
        kind: Option<version::Update>,
    ) -> Result<(), Error>;

    /// Paginate the published contracts. Defaults: strart=0, limit=rest
    fn list_published_contracts(
        &self,
        start: Option<u32>,
        limit: Option<u32>,
    ) -> Result<soroban_sdk::Vec<(soroban_sdk::String, crate::metadata::PublishedContract)>, Error>;
}

#[riff]
pub trait IsDeployable {
    /// Claim a contract id of an already deployed contract
    fn claim_deployed_contract(&mut self, deployed_name: soroban_sdk::String, id: soroban_sdk::Address) -> Result<(), Error>;

    /// Deploys a new published contract returning the deployed contract's id.
    /// If no salt provided it will use the current sequence number.
    fn deploy(
        &mut self,
        contract_name: soroban_sdk::String,
        version: Option<Version>,
        deployed_name: soroban_sdk::String,
        owner: soroban_sdk::Address,
        salt: Option<soroban_sdk::BytesN<32>>,
    ) -> Result<soroban_sdk::Address, Error>;

    /// Fetch contract id
    fn fetch_contract_id(
        &self,
        deployed_name: soroban_sdk::String,
    ) -> Result<soroban_sdk::Address, Error>;

    /// Paginate the deployed contracts. Defaults: strart=0, limit=rest
    fn list_deployed_contracts(
        &self,
        start: Option<u32>,
        limit: Option<u32>,
    ) -> Result<soroban_sdk::Vec<(soroban_sdk::String, soroban_sdk::Address)>, Error>;
}
