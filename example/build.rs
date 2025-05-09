//! This build script generates the test vectors for the example.

use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=../jamtestvectors");
    println!("cargo:rerun-if-changed=../codegen");
    specjam_codegen::run(&Path::new("../jamtestvectors"), &Path::new("src")).unwrap();
}
