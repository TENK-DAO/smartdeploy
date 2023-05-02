use loam_sdk::{
    soroban_sdk::{
        self, contracttype, get_env, Address, BytesN, IntoKey, Map, String,
    },
};

use crate::{
    error::Error,
    metadata::{ContractMetadata, PublishedContract, PublishedWasm},
    version::{self, Version, INITAL_VERSION},
};

use super::IsBinary;

#[contracttype]
#[derive(IntoKey)]
pub struct WasmRegistry(Map<String, PublishedContract>);

impl Default for WasmRegistry {
    fn default() -> Self {
        Self(Map::new(get_env()))
    }
}
impl WasmRegistry {
    pub fn find_contract(&self, name: String) -> Result<PublishedContract, Error> {
        self.0
            .get(name)
            .transpose()
            .unwrap()
            .ok_or(Error::NoSuchContractPublished)
    }

    pub fn find_version(
        &self,
        name: String,
        version: Option<Version>,
    ) -> Result<PublishedWasm, Error> {
        self.find_contract(name)?.get(version)
    }

    pub fn set_contract(&mut self, name: String, contract: PublishedContract) {
        self.0.set(name, contract);
    }
}

impl IsBinary for WasmRegistry {
    fn fetch(
        &self,
        contract_name: String,
        version: Option<Version>,
    ) -> Result<PublishedWasm, Error> {
        self.find_version(contract_name, version)
    }

    fn current_version(&self, contract_name: String) -> Result<Version, Error> {
        self.find_contract(contract_name)?.most_recent_version()
    }

    fn publish(
        &mut self,
        contract_name: String,
        author: Address,
        hash: BytesN<32>,
        repo: Option<String>,
        kind: Option<version::Update>,
    ) -> Result<(), Error> {
        let mut contract = self
            .find_contract(contract_name.clone())
            .unwrap_or_else(|_| PublishedContract::new(author));
        contract.author.require_auth();
        let keys = contract.versions.keys();
        let last_version = keys.last().transpose().unwrap().unwrap_or_default();
        last_version.log();
        let new_version = last_version.clone().update(&kind.unwrap_or_default());
        new_version.log();
        let metadata = if let Some(repo) = repo {
            ContractMetadata { repo }
        } else if new_version == INITAL_VERSION {
            ContractMetadata::default()
        } else {
            contract.get(Some(last_version))?.metadata
        };
        let published_binary = PublishedWasm { hash, metadata };
        contract.versions.set(new_version, published_binary);
        self.set_contract(contract_name, contract);
        Ok(())
    }
}
