use loam_sdk::{
    subcontract,
    soroban_sdk::{self, contracttype, Lazy},
    IntoKey,
};

#[subcontract]
pub trait IsRiff {
    /// Increment increments an internal counter, and returns the value.
    fn increment(&mut self) -> Result<u32, crate::error::Error>;
}

const MAX: u32 = 5;

#[contracttype]
#[derive(IntoKey, Default)]
pub struct Impl(u32);

impl IsRiff for Impl {
    /// Increment increments an internal counter, and returns the value. Errors
    /// if the value is attempted to be incremented past 5.
    fn increment(&mut self) -> Result<u32, crate::error::Error> {
        self.0 += 1;
        if self.0 <= MAX {
            // Return the count to the caller.
            Ok(self.0)
        } else {
            // Return an error if the max is exceeded.
            Err(crate::error::Error::LimitReached)
        }
    }
}
