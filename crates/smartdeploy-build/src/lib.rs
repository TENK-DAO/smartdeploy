use std::path::{Path, PathBuf};

use loam_build::get_target_dir;



#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    LoamBuild(#[from] loam_build::Error),
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

fn manifest() -> PathBuf {
    std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("Cargo.toml")
}

pub fn target_dir() -> Result<PathBuf, Error> {
    let mut target_dir = get_target_dir(&manifest()).map_err(loam_build::Error::Metadata)?;
    target_dir.pop();
    Ok(target_dir.join("smartdeploy"))
}
