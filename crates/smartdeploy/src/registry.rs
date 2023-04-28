use loam_sdk::{
    loam,
    soroban_sdk::{
        self, contracttype, get_env, log, Address, BytesN, Env, IntoKey, Lazy, Map, String,
    },
};

use crate::{
    error::Error,
    metadata::{ContractMetadata, PublishedContract, PublishedWasm},
    util::hash_string,
    version::{self, Version, INITAL_VERSION},
};

#[loam]

pub trait IsBinary {
    /// Fetch the hash from the registry
    fn fetch_hash(
        &self,
        contract_name: String,
        version: Option<Version>,
    ) -> Result<BytesN<32>, Error> {
        Ok(self.fetch(contract_name, version)?.hash)
    }

    fn current_version(&self, contract_name: String) -> Result<Version, Error>;

    /// Fetch the published binary
    fn fetch(
        &self,
        contract_name: String,
        version: Option<Version>,
    ) -> Result<PublishedWasm, Error>;

    fn publish(
        &mut self,
        contract_name: String,
        author: Address,
        hash: BytesN<32>,
        repo: Option<String>,
        kind: Option<version::Kind>,
    ) -> Result<(), Error>;
}

#[loam]
pub trait IsDeployable {
    fn deploy(
        &mut self,
        contract_name: String,
        version: Option<Version>,
        deployed_name: String,
        salt: Option<BytesN<32>>,
    ) -> Result<BytesN<32>, Error>;
}

#[contracttype]
#[derive(IntoKey)]
pub struct ContractRegistry(Map<String, PublishedContract>);

impl Default for ContractRegistry {
    fn default() -> Self {
        Self(Map::new(get_env()))
    }
}
impl ContractRegistry {
    pub fn find_contract(&self, name: String) -> Result<PublishedContract, Error> {
        self.0
            .get(name)
            .transpose()
            .unwrap()
            .ok_or(Error::NoSuchContract)
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
// #[loam]

impl IsBinary for ContractRegistry {
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
        kind: Option<version::Kind>,
    ) -> Result<(), Error> {
        let mut contract = self
            .find_contract(contract_name.clone())
            .unwrap_or_else(|_| PublishedContract::new(author));
        contract.author.require_auth();
        let keys = contract.versions.keys();
        let last_version = keys.last().transpose().unwrap().unwrap_or_default();
        log!(get_env(), "{}", last_version);
        let new_version = last_version.clone().update(&kind.unwrap_or_default());
        let metadata = if let Some(repo) = repo {
            ContractMetadata { repo }
        } else if new_version == INITAL_VERSION {
            ContractMetadata::default()
        } else {
            contract.get(Some(last_version))?.metadata
        };
        let published_binary = PublishedWasm {
            hash,
            metadata,
            num_deployed: 0,
        };
        contract.versions.set(new_version, published_binary);
        self.set_contract(contract_name, contract);
        Ok(())
    }
}

impl IsDeployable for ContractRegistry {
    /// Deploys a new published contract returning the deployed contract's id.
    /// If no salt provided it will use the current sequence number.
    fn deploy(
        &mut self,
        contract_name: String,
        version: Option<Version>,
        deployed_name: String,
        salt: Option<BytesN<32>>,
    ) -> Result<BytesN<32>, Error> {
        let env = get_env();
        let mut binary = self.find_version(contract_name.clone(), version.clone())?;
        let mut contract = self.find_contract(contract_name.clone())?;
        let hash = binary.hash.clone();

        let salt = salt.unwrap_or_else(|| hash_string(env, &deployed_name));
        binary.num_deployed += 1;
        log!(env, "num_deployed {}", binary.num_deployed);
        contract.set(version, binary)?;
        self.set_contract(contract_name, contract);

        Ok(env.deployer().with_current_contract(&salt).deploy(&hash))
    }
}
