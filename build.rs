//! Build script for specjam

use anyhow::Result;

mod codegen;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=jamtestvectors");

    codegen::download()?;
    Ok(())
}
