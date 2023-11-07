use clap::{CommandFactory, Parser};

use smartdeploy_cli::{testnet, Root};

#[tokio::main]
async fn main() {
    std::env::set_var("SOROBAN_CONTRACT_ID", testnet::contract_id());
    std::env::set_var("SOROBAN_RPC_URL", testnet::rpc_url());
    std::env::set_var("SOROBAN_NETWORK_PASSPHRASE", testnet::network_passphrase());
    std::env::remove_var("SOROBAN_NETWORK");
    let mut root = Root::try_parse().unwrap_or_else(|e| {
        let mut cmd = Root::command();
        e.format(&mut cmd).exit();
    });

    if let Err(e) = root.run().await {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
