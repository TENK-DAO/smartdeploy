#![allow(non_upper_case_globals)]
use loam_sdk::soroban_sdk::{
    self, contracttype, env, symbol_short, vec, Address, BytesN, IntoKey, IntoVal, Map, String,
    Val, Vec,
};

use crate::{error::Error, registry::Publishable, util::hash_string, version::Version, Contract};

use super::IsDeployable;

#[contracttype(export = false)]
#[derive(IntoKey)]
pub struct ContractRegistry(pub Map<String, Address>);

impl Default for ContractRegistry {
    fn default() -> Self {
        Self(Map::new(env()))
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
        let env = env();
        if self.0.contains_key(deployed_name.clone()) {
            return Err(Error::NoSuchContractDeployed);
        }
        // signed by owner
        owner.require_auth();
        let hash = Contract::fetch_hash(contract_name, version)?;
        let salt = salt.unwrap_or_else(|| hash_string(&deployed_name));
        // Deploy the contract using the installed WASM code with given hash.
        let id = env.deployer().with_current_contract(salt).deploy(hash);
        // TODO: Invoke using a External API interface that is generated from Core Riff.
        let init_fn = symbol_short!("owner_set");
        let mut init_args: Vec<Val> = Vec::new(env);
        init_args.push_back(owner.into_val(env));
        // Invoke the init function with the given arguments.
        let _res: Val = env.invoke_contract(&id, &init_fn, init_args);
        self.0.set(deployed_name, id.clone());
        Ok(id)
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
