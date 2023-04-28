#![allow(clippy::let_unit_value)]
use loam_sdk::soroban_sdk::{self, contractimpl, set_env, Address, BytesN, Env, String};
use loam_sdk_core_riffs::{Ownable, Redeployable};

use crate::{
    error::Error,
    metadata::PublishedWasm,
    registry::{Binary, Deployable},
    version::Version,
    Contract,
};

pub struct SorobanContract;

#[contractimpl]
impl SorobanContract {
    /// Publish a contract.
    /// Currently a contract's version is a `u32` and publishing will increment it.
    /// If no repo is provided, then the previously published binary's repo will be used. If it's the first
    /// time then it will be empty.
    /// `kind` is Patch by default,
    pub fn publish(
        env: Env,
        contract_name: String,
        author: Address,
        hash: BytesN<32>,
        repo: Option<String>,
        kind: Option<crate::version::Kind>,
    ) -> Result<(), Error> {
        set_env(env);
        Contract::publish(contract_name, author, hash, repo, kind)
    }

    /// Fetch the hash for a given `contract_name`.
    /// If version is not provided, it is the most recent version.
    pub fn fetch(
        env: Env,
        contract_name: String,
        version: Option<Version>,
    ) -> Result<PublishedWasm, Error> {
        set_env(env);
        Contract::fetch(contract_name, version)
    }

    /// Fetch metadata for a given `contract_name`.
    /// If version is not provided, it is the most recent version.
    pub fn fetch_hash(
        env: Env,
        contract_name: String,
        version: Option<Version>,
    ) -> Result<BytesN<32>, Error> {
        set_env(env);
        Contract::fetch_hash(contract_name, version)
    }

    /// Deploy a contract and register a deployed contract
    pub fn deploy(
        env: Env,
        contract_name: String,
        version: Option<Version>,
        deployed_name: String,
        salt: Option<BytesN<32>>,
    ) -> Result<BytesN<32>, Error> {
        set_env(env);
        Contract::deploy(contract_name, version, deployed_name, salt)
    }

    /// Initial method called when contract is deployed. After that only the owner can set the owner, e.i. transfer ownership
    pub fn owner_set(env: Env, owner: Address) {
        set_env(env);
        Contract::owner_set(owner);
    }
    /// Current owner of the contract
    pub fn owner_get(env: Env) -> Option<Address> {
        set_env(env);
        Contract::owner_get()
    }

    /// Redeploy contract to given hash
    pub fn redeploy(env: Env, hash: BytesN<32>) {
        set_env(env);
        Contract::redeploy(hash);
    }
}
