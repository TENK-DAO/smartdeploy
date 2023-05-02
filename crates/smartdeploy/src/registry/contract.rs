use loam_sdk::{
    soroban_sdk::{
        self, contracttype, get_env, Address, BytesN, IntoKey, IntoVal, Map,
        RawVal, String, Symbol, Vec,
    },
};

use crate::{error::Error, registry::Binary, util::hash_string, version::Version, Contract};

use super::IsDeployable;

type ContractId = BytesN<32>;

#[contracttype]
#[derive(IntoKey)]
pub struct ContractRegistry(pub Map<String, ContractId>);

impl Default for ContractRegistry {
    fn default() -> Self {
        Self(Map::new(get_env()))
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
        owner: Address,
        salt: Option<BytesN<32>>,
    ) -> Result<BytesN<32>, Error> {
        let env = get_env();
        if self.0.contains_key(deployed_name.clone()) {
            return Err(Error::NoSuchContractDeployed);
        }
        let hash = Contract::fetch_hash(contract_name.clone(), version.clone())?;
        let salt = salt.unwrap_or_else(|| hash_string(env, &deployed_name));
        // Deploy the contract using the installed WASM code with given hash.
        let id = env.deployer().with_current_contract(&salt).deploy(&hash);
        let init_fn = Symbol::short("owner_set");
        let mut init_args: Vec<RawVal> = Vec::new(env);
        init_args.push_back(owner.into_val(env));
        // Invoke the init function with the given arguments.
        let _res: RawVal = env.invoke_contract(&id, &init_fn, init_args);
        self.0.set(deployed_name, id.clone());
        Ok(id)
    }

    fn fetch_contract_id(&self, deployed_name: String) -> Result<ContractId, Error> {
        self.0
            .get(deployed_name)
            .unwrap()
            .map_err(|_| Error::NoSuchContractDeployed)
    }
}
