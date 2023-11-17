use loam_sdk::soroban_sdk::xdr::{Hash, ScAddress};
use soroban_cli::{
    commands::{config::network, contract::invoke},
    rpc::{self, Client},
};

const CONTRACT_ID: &str = include_str!("./smartdeploy.json");

pub fn contract_id() -> String {
    if let Ok(contract_id) = std::env::var("SMARTDEPLOY_CONTRACT_ID") {
        contract_id
    } else {
        CONTRACT_ID.trim_end().trim_matches('"').to_owned()
    }
}

pub fn contract_id_strkey() -> stellar_strkey::Contract {
    stellar_strkey::Contract::from_string(&contract_id()).unwrap()
}

pub fn contract_address() -> ScAddress {
    ScAddress::Contract(Hash(contract_id_strkey().0))
}

pub fn rpc_url() -> String {
    "https://soroban-testnet.stellar.org:443".to_owned()
}

pub fn network_passphrase() -> String {
    "Test SDF Network ; September 2015".to_owned()
}

pub fn build_invoke_cmd(slop: &[&str]) -> invoke::Cmd {
    invoke::Cmd {
        contract_id: contract_id(),
        wasm: None,
        cost: false,
        unlimited_budget: false,
        slop: slop.iter().map(Into::into).collect(),
        config: soroban_cli::commands::config::Args {
            network: network::Args {
                network: None,
                rpc_url: Some(rpc_url()),
                network_passphrase: Some(network_passphrase()),
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

pub async fn invoke_smartdeploy(slop: &[&str]) -> Result<String, invoke::Error> {
    build_invoke_cmd(slop)
        .run_against_rpc_server(&soroban_cli::commands::global::Args::default())
        .await
}

pub fn client() -> Result<Client, rpc::Error> {
    Client::new(&rpc_url())
}
