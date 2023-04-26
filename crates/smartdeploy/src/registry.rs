use loam_sdk::soroban_sdk::{BytesN, Lazy, String};

use crate::{
    error::Error,
    version::{self, Version},
    PublishedBinary,
};

pub trait Riff: Lazy + Default {}

pub trait ABinary: Riff {
    fn fetch_hash(
        &self,
        contract_name: String,
        version: Option<Version>,
    ) -> Result<BytesN<32>, Error>;

    fn current_version(&self, contract_name: String) -> Result<Version, Error>;

    fn fetch(
        &self,
        contract_name: String,
        version: Option<Version>,
    ) -> Result<PublishedBinary, Error>;

    fn publish(
        &mut self,
        contract_name: String,
        hash: BytesN<32>,
        repo: Option<String>,
        kind: Option<version::Kind>,
    ) -> Result<PublishedBinary, Error>;
}

pub trait Binary {
    type Impl: ABinary;

    fn fetch_hash(contract_name: String, version: Option<Version>) -> BytesN<32> {
        Self::Impl::get_lazy()
            .unwrap_or_default()
            .fetch_hash(contract_name, version)
    }

    fn fetch(contract_name: String, version: Option<Version>) -> PublishedBinary {
        Self::Impl::get_lazy()
            .unwrap_or_default()
            .fetch(contract_name, version)
    }
}
