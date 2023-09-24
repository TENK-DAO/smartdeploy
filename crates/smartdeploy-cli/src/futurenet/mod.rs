const CONTRACT_ID: &str = include_str!("./smartdeploy.json");

pub fn contract_id() -> &'static str {
    CONTRACT_ID.trim_end().trim_matches('"')
}
