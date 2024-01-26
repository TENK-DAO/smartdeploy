use crate::{
    metadata::ContractMetadata,
    version::{Update, Version},
};
use loam_sdk::soroban_sdk::{self, contracttype, Address, Env, IntoVal, String, Val};
use loam_sdk::IntoKey;

#[contracttype]
#[derive(IntoKey)]
pub struct Publish {
    pub published_name: String,
    pub author: Address,
    pub hash: soroban_sdk::BytesN<32>,
    pub repo: ContractMetadata,
    pub kind: Update,
}

#[contracttype]
#[derive(IntoKey)]
pub struct Deploy {
    pub published_name: String,
    pub deployed_name: String,
    pub version: Version,
    pub deployer: Address,
    pub contract_id: Address,
}

pub trait EventPublishable {
    /// Publish an event on the blockchain
    fn publish_event(self, env: &Env);
}

impl<T> EventPublishable for T
where
    T: soroban_sdk::IntoKey + IntoVal<Env, Val>,
{
    fn publish_event(self, env: &Env) {
        env.events().publish((T::into_key(),), self);
    }
}
