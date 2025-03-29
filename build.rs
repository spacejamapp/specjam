//! Build script for specjam

use std::{path::PathBuf, process::Command};

mod codegen;

const VECTORS: &str = "JAM_TEST_VECTORS";

fn main() {
    println!("cargo:rerun-if-changed=codegen");
    println!("cargo:rerun-if-changed=jamtestvectors");
    let workspace =
        PathBuf::from(&std::env::var("CARGO_MANIFEST_DIR").expect("failed to get workspace"));
    let mut vectors = workspace.join("jamtestvectors");
    if let Ok(path) = std::env::var(VECTORS) {
        vectors = PathBuf::from(path);
    }

    if !vectors.exists() {
        println!("cargo:warning=jamtestvectors not found, skipping code generation");
        return;
    }

    // run rustfmt if exists
    if let Err(e) = Command::new("rustfmt")
        .arg(workspace.join("src/registry.rs"))
        .status()
    {
        eprintln!("rustfmt check failed: {}", e);
    }

    // run the codegen
    let registry = workspace.join("src");
    if let Err(e) = codegen::run(&vectors, &registry) {
        eprintln!("failed to run codegen: {}", e);
    }
}
