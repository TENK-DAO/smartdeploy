#![allow(non_upper_case_globals)]
use loam_sdk::soroban_sdk::{self, contracttype, env, vec, Address, BytesN, Map, String, Lazy, Symbol, symbol_short};

use crate::{error::Error, registry::Publishable, util::{hash_string, MAX_BUMP}, version::Version, Contract};

use super::{IsDeployable, IsDevDeployable};


loam_sdk::import_contract!(core_riff);

// Is the same as 

// mod core_riff {
//     use loam_sdk::soroban_sdk;
//     loam_sdk::soroban_sdk::contractimport!(file = "../../target/loam/core_riff.wasm",);
// }


#[contracttype(export = false)]
pub struct ContractRegistry(pub Map<String, Address>);

impl Default for ContractRegistry {
    fn default() -> Self {
        Self(Map::new(env()))
    }
}

fn key() -> Symbol {
    symbol_short!("contractR")
}


impl Lazy for ContractRegistry {
    fn get_lazy() -> Option<Self> {
        env().storage().persistent().get(&key())
    }

    fn set_lazy(self) {
        let key = &key();
        env().storage().persistent().set(key, &self);
        env().storage().persistent().bump(key, MAX_BUMP, MAX_BUMP);
    }
}

impl IsDeployable for ContractRegistry {
    fn claim_deployed_contract(&mut self, deployed_name: String, id: Address) -> Result<(), Error> {
        if self.0.contains_key(deployed_name.clone()) {
            return Err(Error::AlreadyClaimed);
        }
        self.0.set(deployed_name, id);
        Ok(())
    }
    fn deploy(
        &mut self,
        contract_name: String,
        version: Option<Version>,
        deployed_name: String,
        owner: Address,
        salt: Option<BytesN<32>>,
    ) -> Result<Address, Error> {
        if self.0.contains_key(deployed_name.clone()) {
            return Err(Error::NoSuchContractDeployed);
        }
        // signed by owner
        owner.require_auth();
        let hash = Contract::fetch_hash(contract_name, version)?;
        let salt = salt.unwrap_or_else(|| hash_string(&deployed_name));
        let address = deploy_and_init(&owner, salt, hash)?;
        self.0.set(deployed_name, address.clone());
        Ok(address)
    }

    fn fetch_contract_id(&self, deployed_name: String) -> Result<Address, Error> {
        self.0
            .get(deployed_name)
            .ok_or(Error::NoSuchContractDeployed)
    }

    fn list_deployed_contracts(
        &self,
        start: Option<u32>,
        limit: Option<u32>,
    ) -> Result<soroban_sdk::Vec<(soroban_sdk::String, soroban_sdk::Address)>, Error> {
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

fn deploy_and_init(
    owner: &Address,
    salt: BytesN<32>,
    wasm_hash: BytesN<32>,
) -> Result<Address, Error> {
    // Deploy the contract using the installed WASM code with given hash.
    let address = env()
        .deployer()
        .with_current_contract(salt)
        .deploy(wasm_hash);
    // Set the owner of the contract to the given owner.
    let _ = core_riff::Client::new(env(), &address)
        .try_owner_set(owner)
        .map_err(|_| Error::InitFailed)?;
    Ok(address)
}

impl IsDevDeployable for ContractRegistry {
    fn dev_deploy(
        &mut self,
        name: soroban_sdk::String,
        owner: soroban_sdk::Address,
        wasm: soroban_sdk::Bytes,
    ) -> Result<soroban_sdk::Address, Error> {
        let wasm_hash = env().deployer().upload_contract_wasm(wasm);
        if let Some(address) = self.0.get(name.clone()) {
            let contract = core_riff::Client::new(env(), &address);
            contract.redeploy(&wasm_hash);
            return Ok(address);
        }
        let salt = hash_string(&name);
        let id = deploy_and_init(&owner, salt, wasm_hash)?;
        self.0.set(name, id.clone());
        Ok(id)
    }
}
