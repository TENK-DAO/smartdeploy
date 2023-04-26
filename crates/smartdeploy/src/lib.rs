#![no_std]
use loam_sdk::{
    soroban_sdk::{self, contracttype, get_env, log, Address, Bytes, BytesN, Env, Map, String},
    IntoKey,
};

extern crate alloc;
use loam_sdk_core_riffs::{owner::Owner, Ownable, Redeployable};
use version::{Version, INITAL_VERSION};

pub mod error;
pub mod gen;
pub mod registry;
pub mod version;

use error::Error;

#[contracttype]
#[derive(IntoKey, Default)]
pub struct SmartDeploy {
    pub contracts: Contracts,
}

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
#[contracttype]
pub struct ContractMetadata {
    repo: String,
}

impl Default for ContractMetadata {
    fn default() -> Self {
        Self {
            repo: String::from_slice(get_env(), ""),
        }
    }
}

/// Contains
#[contracttype]
#[derive(Clone)]
pub struct PublishedBinary {
    hash: BytesN<32>,
    metadata: ContractMetadata,
    num_deployed: u64,
}

/// Contains
#[contracttype]
#[derive(Clone)]
pub struct PublishedContract {
    versions: Map<Version, PublishedBinary>,
    author: Address,
}

impl PublishedContract {
    pub fn new(author: Address) -> Self {
        Self {
            author,
            versions: Map::new(get_env()),
        }
    }
}

impl PublishedContract {
    pub fn most_recent_version(&self) -> Result<Version, Error> {
        self.versions
            .keys()
            .last()
            .transpose()
            .unwrap()
            .ok_or(Error::NoSuchVersion)
    }

    pub fn get(&self, version: Option<Version>) -> Result<PublishedBinary, Error> {
        let version = if let Some(version) = version {
            version
        } else {
            self.most_recent_version()?
        };
        self.versions
            .get(version)
            .transpose()
            .unwrap()
            .ok_or(Error::NoSuchVersion)
    }

    pub fn set(&mut self, version: Option<Version>, binary: PublishedBinary) -> Result<(), Error> {
        let version = if let Some(version) = version {
            version
        } else {
            self.most_recent_version()?
        };
        self.versions.set(version, binary);
        Ok(())
    }
}

#[contracttype]
#[derive(Clone)]
pub struct Contracts(Map<String, PublishedContract>);

impl Default for Contracts {
    fn default() -> Self {
        Self(Map::new(get_env()))
    }
}

impl Contracts {
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
    ) -> Result<PublishedBinary, Error> {
        self.find_contract(name)?.get(version)
    }

    pub fn set_contract(&mut self, name: String, contract: PublishedContract) {
        self.0.set(name, contract);
    }
}
// #[loam]

impl SmartDeploy {
    /// Register a contract name to allow publishing.
    pub fn register_name(&mut self, author: Address, contract_name: String) -> Result<(), Error> {
        if self.contracts.find_contract(contract_name.clone()).is_ok() {
            return Err(Error::AlreadyRegistered);
        }
        self.contracts
            .set_contract(contract_name, PublishedContract::new(author));
        Ok(())
    }

    /// Publish a contract.
    /// Currently a contract's version is a `u32` and publishing will increment it.
    /// If no repo is provided, then the previously published binary's repo will be used. If it's the first
    /// time then it will be empty.
    /// `kind` is Patch by default,
    pub fn publish_binary(
        &mut self,
        contract_name: String,
        hash: BytesN<32>,
        repo: Option<String>,
        kind: Option<version::Kind>,
    ) -> Result<(), Error> {
        let mut contract = self.contracts.find_contract(contract_name.clone())?;
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
        let published_binary = PublishedBinary {
            hash,
            metadata,
            num_deployed: 0,
        };
        contract.versions.set(new_version, published_binary);
        self.contracts.set_contract(contract_name, contract);
        Ok(())
    }

    /// Fetch the hash for a given `contract_name`.
    /// If version is not provided, it is the most recent version.
    pub fn fetch(
        &self,
        contract_name: String,
        version: Option<Version>,
    ) -> Result<BytesN<32>, Error> {
        Ok(self.contracts.find_version(contract_name, version)?.hash)
    }

    /// Fetch metadata for a given `contract_name`.
    /// If version is not provided, it is the most recent version.
    pub fn fetch_metadata(
        &self,
        contract_name: String,
        version: Option<Version>,
    ) -> Result<ContractMetadata, Error> {
        Ok(self
            .contracts
            .find_version(contract_name, version)?
            .metadata)
    }

    /// Deploys a new published contract returning the deployed contract's id.
    /// If no salt provided it will use the current sequence number.
    pub fn deploy(
        &mut self,
        contract_name: String,
        version: Option<Version>,
        deployed_name: String,
        salt: Option<BytesN<32>>,
    ) -> Result<BytesN<32>, Error> {
        let env = get_env();
        let wasm_hash = self.fetch(contract_name.clone(), version.clone())?;
        let mut contract = self.contracts.find_contract(contract_name.clone())?;
        let mut binary = self
            .contracts
            .find_version(contract_name.clone(), version.clone())?;

        let salt = salt.unwrap_or_else(|| hash_string(env, &deployed_name));
        binary.num_deployed += 1;
        log!(env, "num_deployed {}", binary.num_deployed);
        contract.set(version, binary)?;
        self.contracts.set_contract(contract_name, contract);

        Ok(env
            .deployer()
            .with_current_contract(&salt)
            .deploy(&wasm_hash))
    }

    /// How many deploys have been made for the given contract.
    pub fn get_num_deploys(
        &self,
        contract_name: String,
        version: Option<Version>,
    ) -> Result<u64, Error> {
        let contracts = &self.contracts;
        let binary: PublishedBinary = contracts.find_version(contract_name, version)?;
        Ok(binary.num_deployed)
    }

    pub fn get_contracts(&self) -> Contracts {
        self.contracts.clone()
    }
}

pub fn hash_string(env: &Env, s: &String) -> BytesN<32> {
    let len = s.len() as usize;
    let mut bytes = [0u8; 100];
    let bytes = &mut bytes[0..len];
    s.copy_into_slice(bytes);
    let mut b = Bytes::new(env);
    b.copy_from_slice(0, bytes);
    env.crypto().sha256(&b)
}

impl Ownable for SmartDeploy {
    type Impl = Owner;
}

impl Redeployable for SmartDeploy {}

#[cfg(test)]
mod test;
