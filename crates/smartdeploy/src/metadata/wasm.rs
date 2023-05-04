use loam_sdk::soroban_sdk::{self, contracttype, BytesN};

use super::ContractMetadata;

/// Contains info about specific version of published binary
#[contracttype]
#[derive(Clone)]
pub struct PublishedWasm {
    pub hash: BytesN<32>,
    pub metadata: ContractMetadata,
}
