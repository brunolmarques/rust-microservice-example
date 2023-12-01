use std::{env, path::PathBuf};
use tonic_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .out_dir(out_dir.clone())
        .file_descriptor_set_path(out_dir.join("authentication.bin"))
        .compile(
            &["proto/authentication.proto"], 
            &["proto"]
        )?;
    Ok(())
}