#![no_std]
use loam_sdk::soroban_sdk;

smartdeploy_sdk::dev_deploy!();
#[cfg(feature = "core_riff")]
smartdeploy_sdk::core_riff!();
