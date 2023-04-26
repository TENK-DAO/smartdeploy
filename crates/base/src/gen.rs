use loam_sdk::soroban_sdk::{self, contractimpl, set_env, Address, BytesN, Env};
use loam_sdk_core_riffs::{Ownable, Redeployable};

use crate::Contract;

pub struct SorobanContract;

#[contractimpl]
impl SorobanContract {
    pub fn owner_set(env: Env, owner: Address) {
        set_env(env);
        Contract::owner_set(owner);
    }
    pub fn owner_get(env: Env) -> Option<Address> {
        set_env(env);
        Contract::owner_get()
    }

    pub fn redeploy(env: Env, wasm_hash: BytesN<32>) {
        set_env(env);
        Contract::redeploy(wasm_hash);
    }
}
