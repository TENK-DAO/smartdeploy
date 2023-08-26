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
        pub fn owner_get(env: soroban_sdk::Env) -> Option<soroban_sdk::Address> {
            env.storage().instance().get(&Self::owner_key())
        }
    
        pub fn owner_set(env: soroban_sdk::Env, new_owner: soroban_sdk::Address) {
            Self::owner_get(env.clone()).as_ref().map(soroban_sdk::Address::require_auth);
            env.storage().instance().set(&Self::owner_key(), &new_owner);
        }
    
        pub fn redeploy(env: soroban_sdk::Env, wasm_hash: soroban_sdk::BytesN<32>) {
            Self::owner_get(env.clone()).as_ref().map(Address::require_auth);
            env.deployer().update_current_contract_wasm(wasm_hash);
        }
        fn owner_key() -> soroban_sdk::Symbol {
            soroban_sdk::symbol_short!("owner")
        }
}
    }.into()
}
