const CONTRACT_ID: &str = include_str!("./smartdeploy.json");

pub fn contract_id() -> &'static str {
    CONTRACT_ID.trim_end().trim_matches('"')
}

pub fn rpc_url() -> String {
    "https://soroban-testnet.stellar.org:443".to_owned()
}

pub fn network_passphrase() -> String {
    "Test SDF Network ; September 2015".to_owned()
}