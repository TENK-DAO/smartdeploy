use loam_sdk::soroban_sdk::{self, contracttype, env, Address, Map, String};

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
#[contracttype]
pub struct ContractMetadata {
    pub repo: String,
}

impl Default for ContractMetadata {
    fn default() -> Self {
        Self {
            repo: String::from_slice(env(), ""),
        }
    }
}
//

use crate::{error::Error, version::Version};

use super::PublishedWasm;
/// Contains
#[contracttype]
#[derive(Clone)]
pub struct PublishedContract {
    pub versions: Map<Version, PublishedWasm>,
    pub author: Address,
}

impl PublishedContract {
    pub fn new(author: Address) -> Self {
        Self {
            author,
            versions: Map::new(env()),
        }
    }
}

impl PublishedContract {
    pub fn most_recent_version(&self) -> Result<Version, Error> {
        self.versions
            .keys()
            .last()
            .transpose()
            .unwrap()
            .ok_or(Error::NoSuchVersion)
    }

    pub fn get(&self, version: Option<Version>) -> Result<PublishedWasm, Error> {
        let version = if let Some(version) = version {
            version
        } else {
            self.most_recent_version()?
        };
        self.versions
            .get(version)
            .transpose()
            .unwrap()
            .ok_or(Error::NoSuchVersion)
    }

    pub fn set(&mut self, version: Option<Version>, binary: PublishedWasm) -> Result<(), Error> {
        let version = if let Some(version) = version {
            version
        } else {
            self.most_recent_version()?
        };
        self.versions.set(version, binary);
        Ok(())
    }
}
