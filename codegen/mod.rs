//! Code generation scripts for embedding the jam test vectors into

use anyhow::Result;
use registry::Registry;
use std::{env, fs, path::PathBuf, process::Command};

mod registry;

const REPO: &str = "https://github.com/clearloop/jam-test-vectors.git";
const INTO: &str = "jamtestvectors";

/// The code generator
pub struct Codegen;

impl Codegen {
    /// Run the code generator
    pub fn run() -> Result<()> {
        Self::download()?;

        // get the root and output directories
        let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join(INTO);
        let output = PathBuf::from(env::var("OUT_DIR")?);

        // write the head hash to the output directory
        let head = Self::head()?;
        fs::write(output.join("head.txt"), head.trim())?;

        // generate the test vectors
        Registry::new(&root, &output).run()
    }

    /// Download the jam test vectors
    pub fn download() -> Result<()> {
        let into = PathBuf::from(INTO);
        if into.exists() {
            return Ok(());
        }

        Command::new("git").args(["clone", REPO, INTO]).status()?;
        Ok(())
    }

    fn head() -> Result<String> {
        let hash = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(INTO)
            .output()?
            .stdout;
        Ok(String::from_utf8(hash)?)
    }
}
