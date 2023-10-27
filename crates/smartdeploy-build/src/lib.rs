use std::path::{Path, PathBuf};

use loam_build::get_target_dir;



#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    LoamBuild(#[from] loam_build::Error),
    #[error("Missing contract_id for {0}")]
    MissingContractId(String),
}


pub fn wasm_location(name: &str, out_dir: Option<&Path>) -> Result<PathBuf, Error> {
    let out_dir = if let Some(out_dir) = out_dir {
        out_dir.to_path_buf()
    } else {
        target_dir()?
    };
    let mut out_file = out_dir.join(name).join("index");
    out_file.set_extension("wasm");
    Ok(out_file)
}

pub fn contract_id(name: &str, out_dir: Option<&Path>) -> Result<String, Error> {
    let wasm = wasm_location(name, out_dir)?;
    let parent = wasm.parent().ok_or_else(|| Error::MissingContractId(name.to_owned()))?;
    let id_file = parent.join("contract_id.txt");
    std::fs::read_to_string(id_file).map_err(|_| Error::MissingContractId(name.to_owned()))
}

fn manifest() -> PathBuf {
    std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("Cargo.toml")
}

pub fn target_dir() -> Result<PathBuf, Error> {
    let mut target_dir = get_target_dir(&manifest()).map_err(loam_build::Error::Metadata)?;
    target_dir.pop();
    Ok(target_dir.join("smartdeploy"))
}
