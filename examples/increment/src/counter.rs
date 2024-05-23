use loam_sdk::{
    subcontract,
    soroban_sdk::{self, contracttype, Lazy},
    IntoKey,
};

#[subcontract]
pub trait IsRiff {
    /// Increment increments an internal counter, and returns the value.
    fn increment(&mut self) -> u32;
}

#[contracttype]
#[derive(IntoKey, Default)]
pub struct Impl(u32);

impl IsRiff for Impl {
    /// Increment increments an internal counter, and returns the value.
    fn increment(&mut self) -> u32 {
        self.0 += 1;
        self.0
    }
}
