use clap::Parser;

use soroban_cli::commands::{contract::invoke, global};

#[derive(Parser, Debug, Clone)]
pub struct Cmd {
    pub deployed_name: String,
    #[command(flatten)]
    pub call: invoke::Cmd,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Invoke(#[from] invoke::Error),
}

impl Cmd {
    pub async fn run(&self) -> Result<(), Error> {
        let mut contract_invoke = self.call.clone();
        contract_invoke.slop = vec!["fetch_contract_id", "--deployed_name", &self.deployed_name]
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();
        let global_args = &global::Args::default();
        let id = contract_invoke.invoke(global_args).await?;
        let mut contract = self.call.clone();
        contract.contract_id = id.trim_matches('"').to_string();
        contract.run(global_args).await?;
        Ok(())
    }
}
