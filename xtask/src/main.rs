use std::env::{self, Args};

use soroban_cli::{commands::contract, CommandParser};

type DynError = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() {
    if let Err(e) = try_main().await {
        eprintln!("{e}");
        std::process::exit(-1);
    }
}

#[allow(clippy::single_match_else)]
async fn try_main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("publish") => publish(env::args()).await,
        _ => {
            println!("publish <name> <wasm file>");
            Err("".into())
        }
    }
}

#[allow(clippy::unused_async)]
async fn publish(args: Args) -> Result<(), DynError> {
    let mut args = args.skip(2);
    let name = args.next().unwrap();
    let wasm_file = args.next().unwrap();
    println!("{name} {wasm_file}");
    let cmd = contract::install::Cmd::parse_arg_vec(&[
        "--wasm",
        wasm_file.as_str(),
        "--network=futurenet",
    ])?;
    let wasm_hash = cmd.run_and_get_hash().await?;

    let cmd = contract::invoke::Cmd::parse_arg_vec(
        [""])?;

    // cmd.run_in_sandbox(contract)

    // println!("published {} to {}", name, result);
    Ok(())
}
