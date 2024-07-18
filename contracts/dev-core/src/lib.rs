#![no_std]
use loam_sdk::soroban_sdk;

smartdeploy_sdk::dev_deploy!();
#[cfg(feature = "core_subcontract")]
smartdeploy_sdk::core!();
