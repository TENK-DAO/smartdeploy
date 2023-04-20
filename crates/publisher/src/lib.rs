#![no_std]
use soroban_sdk::{
    self, contracterror, contractimpl, contracttype, log, Address, Bytes, BytesN, Env, Map, String,
    Symbol, TryFromVal, TryIntoVal,
};

extern crate alloc;
use version::{Version, INITAL_VERSION};

pub mod version;

pub struct Publisher;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
#[contracttype]
pub struct ContractMetadata {
    repo: String,
}

/// Contains
#[contracttype]
pub struct PublishedBinary {
    hash: BytesN<32>,
    metadata: ContractMetadata,
    num_deployed: u64,
}

/// Contains
#[contracttype]
pub struct PublishedContract {
    versions: Map<Version, PublishedBinary>,
    author: Address,
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

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// No such Contract has been registered
    NoSuchContract = 1,
    /// No such version of the contact has been published
    NoSuchVersion = 2,
    /// Contract already registered
    AlreadyRegistered = 3,
}

#[contracttype]
struct Contracts(Map<String, PublishedContract>);

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

impl Contracts {
    pub fn key(env: &Env) -> String {
        String::try_from_val(env, &"").unwrap()
    }
}

impl Contracts {
    pub fn get(env: &Env) -> Self {
        let key = &Self::key(env);
        env.storage()
            .get(key)
            .unwrap_or_else(|| Ok(Contracts(Map::new(env))))
            .unwrap()
    }

    pub fn set(env: &Env, contracts: Contracts) {
        env.storage().set(&Self::key(env), &contracts);
    }
}

#[contractimpl]
impl Publisher {
    /// Register a contract name to allow publishing.
    pub fn register_name(env: Env, author: Address, contract_name: String) -> Result<(), Error> {
        let mut this = Contracts::get(&env);
        if this.find_contract(contract_name.clone()).is_ok() {
            return Err(Error::AlreadyRegistered);
        }
        this.set_contract(
            contract_name,
            PublishedContract {
                author,
                versions: Map::new(&env),
            },
        );

        Contracts::set(&env, this);
        Ok(())
    }

    /// Publish a contract.
    /// Currently a contract's version is a `u32` and publishing will increment it.
    /// If no repo is provided, then the previously published binary's repo will be used. If it's the first
    /// time then it will be empty.
    /// `kind` is Patch by default,
    pub fn publish_binary(
        env: Env,
        contract_name: String,
        hash: BytesN<32>,
        repo: Option<String>,
        kind: Option<version::Kind>,
    ) -> Result<(), Error> {
        let mut contracts = Contracts::get(&env);
        let mut contract = contracts.find_contract(contract_name.clone())?;
        contract.author.require_auth();
        let keys = contract.versions.keys();
        let last_version = keys.last().transpose().unwrap().unwrap_or_default();

        let new_version = last_version.update(&kind.unwrap_or_default());
        let metadata = if let Some(repo) = repo {
            ContractMetadata { repo }
        } else if new_version == INITAL_VERSION {
            ContractMetadata {
                repo: String::from_slice(&env, ""),
            }
        } else {
            contract.get(Some(new_version.clone()))?.metadata
        };
        let published_binary = PublishedBinary {
            hash,
            metadata,
            num_deployed: 0,
        };
        contract.versions.set(new_version, published_binary);
        contracts.set_contract(contract_name, contract);
        Contracts::set(&env, contracts);
        Ok(())
    }

    /// Fetch the hash for a given contract_name.
    /// If version is not provided, it is the most recent version.
    pub fn fetch(
        env: Env,
        contract_name: String,
        version: Option<Version>,
    ) -> Result<BytesN<32>, Error> {
        // Err(Error::NoSuchVersion)
        Ok(Contracts::get(&env)
            .find_version(contract_name, version)?
            .hash)
    }

    /// Fetch metadata for a given contract_name.
    /// If version is not provided, it is the most recent version.
    pub fn fetch_metadata(
        env: Env,
        contract_name: String,
        version: Option<Version>,
    ) -> Result<ContractMetadata, Error> {
        Ok(Contracts::get(&env)
            .find_version(contract_name, version)?
            .metadata)
    }

    /// Deploys a new published contract returning the deployed contract's id.
    /// If no salt provided it will use the current sequence number.
    #[allow(clippy::cast_possible_truncation, clippy::needless_range_loop)]
    pub fn deploy(
        env: Env,
        contract_name: String,
        version: Option<Version>,
        deployed_name: String,
        salt: Option<BytesN<32>>,
    ) -> Result<BytesN<32>, Error> {
        let wasm_hash = Self::fetch(env.clone(), contract_name.clone(), version.clone())?;
        let mut contracts = Contracts::get(&env);
        let mut contract = contracts.find_contract(contract_name.clone())?;
        let mut binary = contracts.find_version(contract_name.clone(), version.clone())?;

        let salt = salt.unwrap_or_else(|| hash_string(&env, &deployed_name));
        binary.num_deployed += 1;
        log!(&env, "num_deployed {}", binary.num_deployed);
        contract.set(version, binary)?;
        contracts.set_contract(contract_name, contract);
        Contracts::set(&env, contracts);

        Ok(env
            .deployer()
            .with_current_contract(&salt)
            .deploy(&wasm_hash))
    }

    /// How many deploys have been made for the given contract.
    pub fn get_num_deploys(
        env: Env,
        contract_name: String,
        version: Option<Version>,
    ) -> Result<u64, Error> {
        let contracts = Contracts::get(&env);
        let binary: PublishedBinary = contracts.find_version(contract_name, version)?;
        Ok(binary.num_deployed)
    }

    pub fn hash(env: Env, input: String) -> BytesN<32> {
        hash_string(&env, &input)
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

#[cfg(test)]
mod test;
