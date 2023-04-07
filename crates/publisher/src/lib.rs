#![no_std]

use soroban_sdk::{
    contracterror, contractimpl, contracttype, log, Address, BytesN, Env, Map, String, TryFromVal,
};

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

type ContractVersion = u32;

/// Contains
#[contracttype]
pub struct PublishedContract {
    versions: Map<u32, PublishedBinary>,
    author: Address,
}

impl PublishedContract {
    pub fn most_recent_version(&self) -> Result<ContractVersion, Error> {
        self.versions
            .keys()
            .last()
            .transpose()
            .unwrap()
            .ok_or(Error::NoSuchVersion)
    }

    pub fn get(&self, version: Option<ContractVersion>) -> Result<PublishedBinary, Error> {
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

    pub fn set(
        &mut self,
        version: Option<ContractVersion>,
        binary: PublishedBinary,
    ) -> Result<(), Error> {
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
        version: Option<ContractVersion>,
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
    pub fn publish_binary(
        env: Env,
        contract_name: String,
        hash: BytesN<32>,
        repo: Option<String>,
    ) -> Result<(), Error> {
        let mut contracts = Contracts::get(&env);
        let mut contract = contracts.find_contract(contract_name.clone())?;
        contract.author.require_auth();
        let keys = contract.versions.keys();
        let last_version = keys.last().transpose().unwrap().unwrap_or(0);
        let version = last_version + 1;
        let metadata = if let Some(repo) = repo {
            ContractMetadata { repo }
        } else if version == 1 {
            ContractMetadata {
                repo: String::from_slice(&env, ""),
            }
        } else {
            let last = contract.get(Some(last_version))?;
            last.metadata
        };
        let published_binary = PublishedBinary {
            hash,
            metadata,
            num_deployed: 0,
        };
        contract.versions.set(version, published_binary);
        contracts.set_contract(contract_name, contract);
        Contracts::set(&env, contracts);
        Ok(())
    }

    /// Fetch the hash for a given contract_name.
    /// If version is not provided, it is the most recent version.
    pub fn fetch(
        env: Env,
        contract_name: String,
        version: Option<u32>,
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
        version: Option<u32>,
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
        version: Option<u32>,
    ) -> Result<BytesN<32>, Error> {
        let wasm_hash = Self::fetch(env.clone(), contract_name.clone(), version)?;
        let mut contracts = Contracts::get(&env);
        let mut contract = contracts.find_contract(contract_name.clone())?;
        let mut binary = contracts.find_version(contract_name.clone(), version)?;

        let salt_bytes: [u8; 8] = binary.num_deployed.to_be_bytes();

        log!(&env, "num_deployed {}", binary.num_deployed);
        binary.num_deployed += 1;
        contract.set(version, binary)?;
        contracts.set_contract(contract_name, contract);
        Contracts::set(&env, contracts);
        let mut salt = [0; 32];
        salt[..8].copy_from_slice(&salt_bytes[..8]);

        Ok(env
            .deployer()
            .with_current_contract(&salt)
            .deploy(&wasm_hash))
    }

    pub fn get_num(env: Env, contract_name: String) -> Result<u64, Error> {
        let contracts = Contracts::get(&env);
        let binary = contracts.find_version(contract_name, None)?;
        Ok(binary.num_deployed)
    }
}

mod test;
