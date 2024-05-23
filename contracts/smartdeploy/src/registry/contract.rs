#![allow(non_upper_case_globals)]
use loam_sdk::soroban_sdk::{
    self, contracttype, env, symbol_short, vec, Address, BytesN, Lazy, Map, String, Symbol, Val,
};

use crate::{
    error::Error,
    events::{Claim, Deploy, EventPublishable},
    registry::Publishable,
    util::{hash_string, MAX_BUMP},
    version::Version,
    Contract, WasmRegistry,
};

use super::{IsClaimable, IsDeployable, IsDevDeployable};

loam_sdk::import_contract!(core_subcontract);

// Is the same as

// mod core_subcontract {
//     use loam_sdk::soroban_sdk;
//     loam_sdk::soroban_sdk::contractimport!(file = "../../target/loam/core_subcontract.wasm",);
// }

#[contracttype(export = false)]
pub struct ContractRegistry(pub Map<String, ContractType>);

#[contracttype(export = false)]
#[derive(Clone)]
pub enum ContractType {
    ContractById(Address),
    ContractByIdAndAdmin(Address, Address),
}

impl ContractType {
    pub fn contract_id(&self) -> &Address {
        match self {
            Self::ContractById(id) | Self::ContractByIdAndAdmin(id, _) => id,
        }
    }
    pub fn admin(&self) -> Option<&Address> {
        match self {
            Self::ContractByIdAndAdmin(_, admin) => Some(admin),
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
        env()
            .storage()
            .persistent()
            .extend_ttl(key, MAX_BUMP, MAX_BUMP);
    }
}

impl IsDeployable for ContractRegistry {
    fn deploy(
        &mut self,
        contract_name: String,
        version: Option<Version>,
        deployed_name: String,
        admin: Address,
        salt: Option<BytesN<32>>,
        init: Option<(Symbol, soroban_sdk::Vec<soroban_sdk::Val>)>,
    ) -> Result<Address, Error> {
        if self.0.contains_key(deployed_name.clone()) {
            return Err(Error::NoSuchContractDeployed);
        }
        // signed by Admin
        admin.require_auth();
        let hash = Contract::fetch_hash(contract_name.clone(), version.clone())?;
        let salt = salt.unwrap_or_else(|| hash_string(&deployed_name));
        let address = deploy_and_init(&admin, salt, hash)?;
        if let Some((init_fn, args)) = init {
            let _ = env().invoke_contract::<Val>(&address, &init_fn, args);
        }
        self.0.set(
            deployed_name.clone(),
            ContractType::ContractById(address.clone()),
        );

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
            deployer: admin,
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
        admin: soroban_sdk::Address,
    ) -> Result<(), Error> {
        admin.require_auth();
        if self.0.contains_key(deployed_name.clone()) {
            return Err(Error::AlreadyClaimed);
        }
        self.0.set(
            deployed_name.clone(),
            ContractType::ContractByIdAndAdmin(id.clone(), admin.clone()),
        );

        // Publish a Claim event
        Claim {
            deployed_name,
            claimer: admin,
            contract_id: id,
        }
        .publish_event(env());
        Ok(())
    }

    fn get_claimed_admin(
        &self,
        deployed_name: soroban_sdk::String,
    ) -> Result<Option<Address>, Error> {
        self.0
            .get(deployed_name)
            .ok_or(Error::NoSuchContractDeployed)
            .map(|contract| contract.admin().cloned())
    }

    fn redeploy_claimed_contract(
        &self,
        binary_name: Option<soroban_sdk::String>,
        version: Option<Version>,
        deployed_name: soroban_sdk::String,
        redeploy_fn: Option<(soroban_sdk::Symbol, soroban_sdk::Vec<soroban_sdk::Val>)>,
    ) -> Result<(), Error> {
        self.get_claimed_admin(deployed_name.clone())?
            .ok_or(Error::NoAdminSet)?
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
    admin: &Address,
    salt: BytesN<32>,
    wasm_hash: BytesN<32>,
) -> Result<Address, Error> {
    // Deploy the contract using the installed Wasm code with given hash.
    let address = env()
        .deployer()
        .with_current_contract(salt)
        .deploy(wasm_hash);
    // Set the Admin of the contract to the given Admin.
    let _ = core_subcontract::Client::new(env(), &address)
        .try_Admin_set(admin)
        .map_err(|_| Error::InitFailed)?;
    Ok(address)
}

impl IsDevDeployable for ContractRegistry {
    fn dev_deploy(
        &mut self,
        name: soroban_sdk::String,
        admin: soroban_sdk::Address,
        wasm: soroban_sdk::Bytes,
    ) -> Result<soroban_sdk::Address, Error> {
        let wasm_hash = env().deployer().upload_contract_wasm(wasm);
        if let Some(contract_state) = self.0.get(name.clone()) {
            let address = contract_state.contract_id();
            let contract = core_subcontract::Client::new(env(), address);
            contract.redeploy(&wasm_hash);
            return Ok(address.clone());
        }
        let salt = hash_string(&name);
        let id = deploy_and_init(&admin, salt, wasm_hash)?;
        self.0.set(name, ContractType::ContractById(id.clone()));
        Ok(id)
    }
}
