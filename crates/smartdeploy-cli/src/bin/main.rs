use clap::{CommandFactory, Parser};

use smartdeploy_cli::Root;

const CONTRACT_ID: &str = include_str!("../futurenet/smartdeploy.json");

#[tokio::main]
async fn main() {
    let contract_id = CONTRACT_ID.trim_end().trim_matches('"');
    std::env::set_var("SOROBAN_CONTRACT_ID", contract_id);
    std::env::set_var("SOROBAN_NETWORK", "futurenet");
    let mut root = Root::try_parse().unwrap_or_else(|e| {
        let mut cmd = Root::command();
        e.format(&mut cmd).exit();
    });

    if let Err(e) = root.run().await {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
