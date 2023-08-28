extern crate proc_macro;
use proc_macro::TokenStream;

/// Adds `owner_get`, `owner_set`, and `redeploy` to the contract.
/// This way it ensures that it is redeployable by the owner.
#[proc_macro]
pub fn core_riff(_: TokenStream) -> TokenStream {
    quote::quote! {
#[soroban_sdk::contract]
pub struct Contract__;

#[soroban_sdk::contractimpl]
impl Contract__ {
        /// Returns the owner of the contract
        pub fn owner_get(env: soroban_sdk::Env) -> Option<soroban_sdk::Address> {
            env.storage().instance().get(&Self::owner_key())
        }
    
        /// Sets the owner of the contract. If one already set it transfers it to the new owner, if signed by owner.
        pub fn owner_set(env: soroban_sdk::Env, new_owner: soroban_sdk::Address) {
            Self::owner_get(env.clone()).as_ref().map(soroban_sdk::Address::require_auth);
            env.storage().instance().set(&Self::owner_key(), &new_owner);
        }
    
        /// Redeploy the contract to a wasm hash or wasm bytes
        pub fn redeploy(env: soroban_sdk::Env, wasm_hash: soroban_sdk::BytesN<32>) {
            Self::owner_get(env.clone()).as_ref().map(soroban_sdk::Address::require_auth);
            env.deployer().update_current_contract_wasm(wasm_hash);
        }
        fn owner_key() -> soroban_sdk::Symbol {
            soroban_sdk::symbol_short!("owner")
        }
}
    }.into()
}


/// Adds `dev_deploy` to the contract.
/// This way the contract can be redeployed with the supplied bytes
#[proc_macro]
pub fn dev_deploy(_: TokenStream) -> TokenStream {
    quote::quote! {
#[soroban_sdk::contract]
pub struct DevDeploy__;

#[soroban_sdk::contractimpl]
impl DevDeploy__ {
    /// Redeploy the contract to the given wasm bytes
    pub fn dev_deploy(env: soroban_sdk::Env, wasm: loam_sdk::soroban_sdk::Bytes) {
        let wasm_hash = env.deployer().upload_contract_wasm(wasm);
        env.deployer().update_current_contract_wasm(wasm_hash);
    }
}
    }.into()
}
