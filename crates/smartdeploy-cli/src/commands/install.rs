use std::path::PathBuf;

use clap::Parser;

use smartdeploy_build::{target_dir, wasm_location};
use soroban_cli::commands::{
    config::network,
    contract::{fetch, invoke},
    global,
};

use crate::testnet;

#[derive(Parser, Debug, Clone)]
pub struct Cmd {
    /// Name of deployed contract
    pub deployed_name: String,
    /// Where to place the Wasm file. Default `<root>/target/smartdeploy/<deployed_name>/index.wasm`
    #[arg(long, short = 'o')]
    pub out_dir: Option<PathBuf>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Fetch(#[from] fetch::Error),
    #[error(transparent)]
    Invoke(#[from] invoke::Error),
    #[error(transparent)]
    SmartdeployBuild(#[from] smartdeploy_build::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl Cmd {
    pub async fn run(&self) -> Result<(), Error> {
        let contract_id = testnet::contract_id();
        let network = network::Args {
            rpc_url: Some(testnet::rpc_url()),
            network_passphrase: Some(testnet::network_passphrase()),
            ..Default::default()
        };
        let mut cmd = invoke::Cmd {
            contract_id: contract_id.to_string(),
            config: soroban_cli::commands::config::Args {
                network: network.clone(),
                ..Default::default()
            },
            ..Default::default()
        };
        cmd.slop = vec!["fetch_contract_id", "--deployed_name", &self.deployed_name]
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();
        let id = cmd.invoke(&global::Args::default()).await?;
        let contract_id = id.trim_matches('"');
        let out_dir = if let Some(out_dir) = self.out_dir.clone() {
            out_dir
        } else {
            target_dir()?
        };
        let out_file = wasm_location(&self.deployed_name, Some(&out_dir))?;
        let id_file = out_file.parent().unwrap().join("contract_id.txt");
        let fetch_cmd = fetch::Cmd {
            contract_id: contract_id.to_owned(),
            out_file: Some(out_file),
            network,
            ..Default::default()
        };
        fetch_cmd.run().await?;
        std::fs::write(id_file, contract_id)?;
        Ok(())
    }
}
