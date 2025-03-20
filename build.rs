//! Build script for specjam

use std::{fs, path::PathBuf};

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
        println!("cargo:warning=jamtestvectors not found, skipping");
        return;
    }

    // write the head hash to the output file
    let head = codegen::head(&vectors).expect("failed to get head hash");
    let head_path = workspace.join("head.txt");
    if !head_path.exists() {
        fs::File::create(&head_path).expect("failed to create head file");
    }

    fs::write(head_path, head).expect("failed to write head hash");

    // run the codegen
    let registry = workspace.join("src");
    if let Err(e) = codegen::run(&vectors, &registry) {
        eprintln!("failed to run codegen: {}", e);
    }
}
