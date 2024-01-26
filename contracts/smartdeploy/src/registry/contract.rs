#![allow(non_upper_case_globals)]
use loam_sdk::soroban_sdk::{
    self, contracttype, env, symbol_short, vec, Address, BytesN, Lazy, Map, String, Symbol, Val,
};

use crate::{
    error::Error,
    events::{Deploy, EventPublishable},
    registry::Publishable,
    util::{hash_string, MAX_BUMP},
    version::Version,
    Contract,
    WasmRegistry,
};

use super::{IsClaimable, IsDeployable, IsDevDeployable};

loam_sdk::import_contract!(core_riff);

// Is the same as

// mod core_riff {
//     use loam_sdk::soroban_sdk;
//     loam_sdk::soroban_sdk::contractimport!(file = "../../target/loam/core_riff.wasm",);
// }


#[contracttype(export = false)]
#[derive(Clone)]
pub enum ContractType {
    ContractById(Address),
    ContractByIdAndOwner(Address, Address),
}

impl ContractType {
    pub fn contract_id(&self) -> &Address {
        match self {
            Self::ContractById(id) | Self::ContractByIdAndOwner(id, _) => id,
        }
    }
    pub fn owner(&self) -> Option<&Address> {
        match self {
            Self::ContractByIdAndOwner(_, owner) => Some(owner),
            Self::ContractById(_) => None,
        }
    }
}

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
        env().storage().persistent().extend_ttl(key, MAX_BUMP, MAX_BUMP);
    }
}

impl IsDeployable for ContractRegistry {
    fn deploy(
        &mut self,
        contract_name: String,
        version: Option<Version>,
        deployed_name: String,
        owner: Address,
        salt: Option<BytesN<32>>,
        init: Option<(Symbol, soroban_sdk::Vec<soroban_sdk::Val>)>,
    ) -> Result<Address, Error> {
        if self.0.contains_key(deployed_name.clone()) {
            return Err(Error::NoSuchContractDeployed);
        }
        // signed by owner
        owner.require_auth();
        let hash = Contract::fetch_hash(contract_name.clone(), version.clone())?;
        let salt = salt.unwrap_or_else(|| hash_string(&deployed_name));
        let address = deploy_and_init(&owner, salt, hash)?;
        if let  Some((init_fn, args)) = init {
            let _ = env().invoke_contract::<Val>(&address, &init_fn, args);
        }
        self.0.set(deployed_name.clone(), ContractType::ContractById(address.clone()));

        // Publish a deploy event
        let version = version.map_or_else(
            || {
                let published_contract = WasmRegistry::get_lazy()
                    .unwrap()
                    .find_contract(contract_name.clone())?;
                published_contract.most_recent_version()
            },
            Ok,
        )?;
        Deploy {
            published_name: contract_name,
            deployed_name,
            version,
            deployer: owner,
            contract_id: address.clone(),
        }
        .publish_event(env());

        Ok(address)
    }

    fn fetch_contract_id(&self, deployed_name: String) -> Result<Address, Error> {
        self.0
            .get(deployed_name)
            .ok_or(Error::NoSuchContractDeployed)
            .map(|contract| contract.contract_id().clone())
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
            res.push_back((item.0, item.1.contract_id().clone()));
        }
        Ok(res)
    }
}

impl IsClaimable for ContractRegistry {
    fn claim_already_deployed_contract(
        &mut self,
        deployed_name: soroban_sdk::String,
        id: soroban_sdk::Address,
        owner: soroban_sdk::Address,
    ) -> Result<(), Error> {
        owner.require_auth();
        if self.0.contains_key(deployed_name.clone()) {
            return Err(Error::AlreadyClaimed);
        }
        self.0.set(deployed_name, ContractType::ContractByIdAndOwner(id, owner));
        Ok(())
    }

    fn get_claimed_owner(
        &self,
        deployed_name: soroban_sdk::String
    ) -> Result<Option<Address>, Error> {
        self.0
            .get(deployed_name)
            .ok_or(Error::NoSuchContractDeployed)
            .map(|contract| contract.owner().cloned())
    }

    fn redeploy_claimed_contract(
        &self,
        binary_name: Option<soroban_sdk::String>,
        version: Option<Version>,
        deployed_name: soroban_sdk::String,
        redeploy_fn: Option<(soroban_sdk::Symbol, soroban_sdk::Vec<soroban_sdk::Val>)>,
    ) -> Result<(), Error> {
        self.get_claimed_owner(deployed_name.clone())?
            .ok_or(Error::NoOwnerSet)?
            .require_auth();
        let contract_id = self.fetch_contract_id(deployed_name)?;
        if let Some(binary_name) = binary_name {
            let hash = Contract::fetch_hash(binary_name, version)?;
            env().deployer().update_current_contract_wasm(hash);
        } else if let Some((fn_name, args)) = redeploy_fn {
            let _ = env().invoke_contract::<Val>(&contract_id, &fn_name, args);
        } else {
            return Err(Error::RedeployDeployedFailed);
        }
        Ok(())
    }
}

fn deploy_and_init(
    owner: &Address,
    salt: BytesN<32>,
    wasm_hash: BytesN<32>,
) -> Result<Address, Error> {
    // Deploy the contract using the installed Wasm code with given hash.
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
        if let Some(contract_state) = self.0.get(name.clone()) {
            let address = contract_state.contract_id();
            let contract = core_riff::Client::new(env(), address);
            contract.redeploy(&wasm_hash);
            return Ok(address.clone());
        }
        let salt = hash_string(&name);
        let id = deploy_and_init(&owner, salt, wasm_hash)?;
        self.0.set(name, ContractType::ContractById(id.clone()));
        Ok(id)
    }
}
