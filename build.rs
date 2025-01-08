//! Build script for specjam

use anyhow::Result;
use codegen::Codegen;

mod codegen;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=jamtestvectors");
    Codegen::run()
}
