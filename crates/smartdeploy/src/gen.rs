#![allow(clippy::let_unit_value)]
use loam_sdk::soroban_sdk::{self, contractimpl, set_env, Address, BytesN, Env, Lazy, String};
use loam_sdk_core_riffs::{Ownable, Redeployable};

use crate::{error::Error, version::Version, ContractMetadata, SmartDeploy};

pub struct SorobanContract;

#[contractimpl]
impl SorobanContract {
    /// Register a contract name to allow publishing.

    pub fn register_name(env: Env, author: Address, contract_name: String) -> Result<(), Error> {
        set_env(env);
        let mut this = SmartDeploy::get_lazy().unwrap_or_default();
        let res = this.register_name(author, contract_name)?;
        SmartDeploy::set_lazy(this);
        Ok(res)
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
        kind: Option<crate::version::Kind>,
    ) -> Result<(), Error> {
        set_env(env);
        let mut this = SmartDeploy::get_lazy().unwrap_or_default();
        let res = this.publish_binary(contract_name, hash, repo, kind)?;
        SmartDeploy::set_lazy(this);
        Ok(res)
    }

    /// Fetch the hash for a given `contract_name`.
    /// If version is not provided, it is the most recent version.
    pub fn fetch(
        env: Env,
        contract_name: String,
        version: Option<Version>,
    ) -> Result<BytesN<32>, Error> {
        set_env(env);
        SmartDeploy::get_lazy()
            .unwrap_or_default()
            .fetch(contract_name, version)
    }

    /// Fetch metadata for a given `contract_name`.
    /// If version is not provided, it is the most recent version.
    pub fn fetch_metadata(
        env: Env,
        contract_name: String,
        version: Option<Version>,
    ) -> Result<ContractMetadata, Error> {
        set_env(env);
        SmartDeploy::get_lazy()
            .unwrap_or_default()
            .fetch_metadata(contract_name, version)
    }

    pub fn deploy(
        env: Env,
        contract_name: String,
        version: Option<Version>,
        deployed_name: String,
        salt: Option<BytesN<32>>,
    ) -> Result<BytesN<32>, Error> {
        set_env(env);
        let mut this = SmartDeploy::get_lazy().unwrap_or_default();
        let res = this.deploy(contract_name, version, deployed_name, salt)?;
        SmartDeploy::set_lazy(this);
        Ok(res)
    }

    /// How many deploys have been made for the given contract.
    pub fn get_num_deploys(
        env: Env,
        contract_name: String,
        version: Option<Version>,
    ) -> Result<u64, Error> {
        set_env(env);
        SmartDeploy::get_lazy()
            .unwrap_or_default()
            .get_num_deploys(contract_name, version)
    }

    /// Initial method called when contract is deployed. After that only the owner can set the owner, e.i. transfer ownership
    pub fn owner_set(env: Env, owner: Address) {
        set_env(env);
        SmartDeploy::owner_set(owner);
    }
    /// Current owner of the contract
    pub fn owner_get(env: Env) -> Option<Address> {
        set_env(env);
        SmartDeploy::owner_get()
    }

    /// Redeploy contract to given hash
    pub fn redeploy(env: Env, hash: BytesN<32>) {
        set_env(env);
        SmartDeploy::redeploy(hash);
    }
}
