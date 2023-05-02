use loam_sdk::soroban_sdk::{self, contracttype, get_env, Address, BytesN, Map, String};

use crate::{error::Error, version::Version};

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
#[contracttype]
pub struct Contract {
    pub repo: String,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            repo: String::from_slice(get_env(), ""),
        }
    }
}
