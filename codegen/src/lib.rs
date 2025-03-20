//! Code generation scripts for embedding the jam test vectors into

use anyhow::Result;
pub use registry::Registry;
use std::{path::Path, process::Command};

mod registry;

/// The repo of the jam test vectors
pub const REPO: &str = "https://github.com/spacejamapp/jam-test-vectors.git";

/// The scale of the test vectors
#[cfg(not(feature = "full"))]
pub const SCALE: &[&str] = &["tiny"];

/// The scale of the test vectors
#[cfg(feature = "full")]
pub const SCALE: &[&str] = &["tiny", "full"];

/// Run the code generator
pub fn run(vectors: &Path, output: &Path) -> Result<()> {
    Registry::new(vectors, output).run()
}

/// Download the jam test vectors
pub fn download(target: &Path) -> Result<()> {
    if target.exists() {
        return Ok(());
    }

    Command::new("git")
        .args([
            "clone",
            REPO,
            target.to_str().expect("target is not a valid path"),
            "--depth=1",
        ])
        .status()?;
    Ok(())
}

/// Get the head hash of the test vectors
pub fn head(target: &Path) -> Result<String> {
    let hash = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(target)
        .output()?
        .stdout;
    Ok(String::from_utf8(hash)?)
}
