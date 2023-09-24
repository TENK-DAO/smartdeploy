use clap::{CommandFactory, Parser};

use smartdeploy_cli::{Root, futurenet};


#[tokio::main]
async fn main() {
    std::env::set_var("SOROBAN_CONTRACT_ID", futurenet::contract_id());
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
