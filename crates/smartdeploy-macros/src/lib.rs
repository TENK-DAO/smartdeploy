extern crate proc_macro;
use proc_macro::TokenStream;

/// Adds `Admin_get`, `Admin_set`, and `redeploy` to the contract.
/// This way it ensures that it is redeployable by the Admin.
#[proc_macro]
pub fn core(_: TokenStream) -> TokenStream {
    quote::quote! {
    #[soroban_sdk::contract]
    pub struct Contract__;

    #[soroban_sdk::contractimpl]
    impl Contract__ {
            /// Returns the Admin of the contract
            pub fn Admin_get(env: soroban_sdk::Env) -> Option<soroban_sdk::Address> {
                env.storage().instance().get(&Self::Admin_key())
            }

            /// Sets the Admin of the contract. If one already set it transfers it to the new Admin, if signed by Admin.
            pub fn Admin_set(env: soroban_sdk::Env, new_Admin: soroban_sdk::Address) {
                Self::Admin_get(env.clone()).as_ref().map(soroban_sdk::Address::require_auth);
                env.storage().instance().set(&Self::Admin_key(), &new_Admin);
            }

            /// Redeploy the contract to a Wasm hash
            pub fn redeploy(env: soroban_sdk::Env, wasm_hash: soroban_sdk::BytesN<32>) {
                Self::Admin_get(env.clone()).as_ref().map(soroban_sdk::Address::require_auth);
                env.deployer().update_current_contract_wasm(wasm_hash);
            }
            fn Admin_key() -> soroban_sdk::Symbol {
                soroban_sdk::symbol_short!("Admin")
            }
    }
        }
    .into()
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
        /// Redeploy the contract with the given Wasm bytes
        pub fn dev_deploy(env: soroban_sdk::Env, wasm: soroban_sdk::Bytes) {
            let wasm_hash = env.deployer().upload_contract_wasm(wasm);
            env.deployer().update_current_contract_wasm(wasm_hash);
        }
    }
        }
    .into()
}

/// Generates a contract Client for a given contract.
/// It is expected that the name should be the same as the published contract or a contract in your current workspace.
#[proc_macro]
pub fn import_contract_interface(tokens: TokenStream) -> TokenStream {
    let (name, file, _) = file_and_id(&tokens);
    quote::quote! {
        mod #name {
            use super::*;
            use soroban_sdk::TryFromVal;
            soroban_sdk::contractimport!(file = #file);
        }
    }
    .into()
}

/// Generates a contract Client for a given contract.
/// It is expected that the name should be the same as the published contract or a contract in your current workspace.
#[proc_macro]
pub fn import_contract(tokens: TokenStream) -> TokenStream {
    let (name, file, Some(id)) = file_and_id(&tokens) else {
        panic!("contract_id.txt not found")
    };

    quote::quote! {
        mod #name {
            use super::*;
            use soroban_sdk::TryFromVal;
            soroban_sdk::contractimport!(file = #file);

            pub fn address(env: &Env) -> soroban_sdk::Address {
                let bytes: soroban_sdk::BytesN<32> = soroban_sdk::Bytes::from_slice(&env, &[#(#id),*]).try_into().unwrap();
                soroban_sdk::Address::from_contract_id(&bytes)
            }

            pub fn new(env: &Env) -> Client {
                let contract_id =  &address(env);
                Client::new(env, contract_id)
            }
        }
    }
    .into()
}

fn file_and_id(tokens: &TokenStream) -> (syn::Ident, String, Option<[u8; 32]>) {
    let dir = smartdeploy_build::target_dir().expect("failed to find target_dir");
    let wasm = smartdeploy_build::wasm_location(&tokens.to_string(), Some(&dir))
        .expect("failed to file wasm");
    let name =
        syn::parse::<syn::Ident>(tokens.clone()).expect("The input must be a valid identifier");
    let binding = wasm.canonicalize().expect("cannot canonicalize");
    let file = binding.to_str().unwrap();
    let id = std::fs::read_to_string(binding.parent().unwrap().join("contract_id.txt"))
        .ok()
        .and_then(|s| stellar_strkey::Contract::from_string(&s).ok());
    (name, file.to_string(), id.map(|id| id.0))
}

// pub fn contract_id(env: &Env) -> Address {
//     let binding = soroban_sdk::short_symbol!(#id);
//     let s = binding.as_val();
//     Address::try_from_val(&env, s).unwrap()
// }
