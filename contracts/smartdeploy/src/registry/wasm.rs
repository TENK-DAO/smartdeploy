use loam_sdk::soroban_sdk::{
    self, contracttype, env, symbol_short, vec, Address, Lazy, Map, String, Symbol,
};

use crate::{
    error::Error,
    events::{EventPublishable, Publish},
    metadata::{ContractMetadata, PublishedContract, PublishedWasm},
    util::MAX_BUMP,
    version::{self, Version, INITAL_VERSION},
};

use super::IsPublishable;

#[contracttype(export = false)]

pub struct WasmRegistry(Map<String, PublishedContract>);

fn key() -> Symbol {
    symbol_short!("wasmReg")
}

impl Lazy for WasmRegistry {
    fn get_lazy() -> Option<Self> {
        env().storage().persistent().get(&key())
    }

    fn set_lazy(self) {
        let key = &key();
        env().storage().persistent().set(key, &self);
        env().storage().persistent().extend_ttl(key, MAX_BUMP, MAX_BUMP);
    }
}

impl Default for WasmRegistry {
    fn default() -> Self {
        Self(Map::new(env()))
    }
}
impl WasmRegistry {
    pub fn find_contract(&self, name: String) -> Result<PublishedContract, Error> {
        self.0.get(name).ok_or(Error::NoSuchContractPublished)
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

impl IsPublishable for WasmRegistry {
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
        wasm: soroban_sdk::Bytes,
        repo: Option<String>,
        kind: Option<version::Update>,
    ) -> Result<(), Error> {
        let mut contract = self
            .find_contract(contract_name.clone())
            .unwrap_or_else(|_| PublishedContract::new(author.clone()));
        contract.author.require_auth();
        let keys = contract.versions.keys();
        let last_version = keys.last().unwrap_or_default();

        last_version.log();
        let new_version = last_version.clone().update(&kind.clone().unwrap_or_default());
        new_version.log();

        let metadata = if let Some(repo) = repo {
            ContractMetadata { repo }
        } else if new_version == INITAL_VERSION {
            ContractMetadata::default()
        } else {
            contract.get(Some(last_version))?.metadata
        };
        let hash = env().deployer().upload_contract_wasm(wasm);
        let published_binary = PublishedWasm { hash: hash.clone(), metadata: metadata.clone() };
        contract.versions.set(new_version, published_binary);
        self.set_contract(contract_name.clone(), contract);

        // Publish a publish event
        Publish {
            published_name: contract_name,
            author,
            hash,
            repo: metadata,
            kind: kind.unwrap_or_default(),
        }
        .publish_event(env());

        Ok(())
    }

    fn list_published_contracts(
        &self,
        start: Option<u32>,
        limit: Option<u32>,
    ) -> Result<soroban_sdk::Vec<(soroban_sdk::String, crate::metadata::PublishedContract)>, Error>
    {
        let items = self
            .0
            .iter()
            .skip(start.unwrap_or_default() as usize)
            .take(limit.unwrap_or_else(|| self.0.len()) as usize);
        let mut res = vec![env()];
        for item in items {
            res.push_back(item);
        }
        Ok(res)
    }
}
