use core::fmt::Display;

use loam_sdk::soroban_sdk::{self, contracttype};

/// Represents the version of the contract
#[contracttype]
#[derive(Default, Eq, PartialEq, Clone, Debug)]
pub struct Version {
    patch: u32,
    minor: u32,
    major: u32,
}

pub const INITAL_VERSION: Version = Version {
    major: 0,
    minor: 0,
    patch: 1,
};

impl Display for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "v{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}

impl Version {
    #[must_use]
    pub fn publish_patch(mut self) -> Self {
        self.patch += 1;
        self
    }

    #[must_use]
    pub fn publish_minor(mut self) -> Self {
        self.minor += 1;
        self.patch = 0;
        self
    }
    #[must_use]
    pub fn publish_major(mut self) -> Self {
        self.major += 1;
        self.minor = 0;
        self.patch = 0;
        self
    }

    #[must_use]
    pub fn update(self, kind: &Kind) -> Self {
        match kind {
            Kind::Patch => self.publish_patch(),
            Kind::Minor => self.publish_minor(),
            Kind::Major => self.publish_major(),
        }
    }
    pub fn patch(&self) -> u32 {
        self.patch
    }

    pub fn minor(&self) -> u32 {
        self.minor
    }

    pub fn major(&self) -> u32 {
        self.major
    }
}

#[contracttype]
#[derive(Default)]
pub enum Kind {
    #[default]
    Patch,
    Minor,
    Major,
}
