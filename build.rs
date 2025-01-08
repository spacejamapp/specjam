//! Build script for specjam

use anyhow::Result;
use codegen::Codegen;

mod codegen;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=jamtestvectors");
    Codegen::run()?;
    if std::env::var("CLEAN_VECTORS").is_ok() {
        std::fs::remove_dir_all("jamtestvectors")?;
    }
    Ok(())
}
