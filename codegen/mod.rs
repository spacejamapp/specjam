//! Code generation scripts for embedding the jam test vectors into

use anyhow::Result;
use registry::Registry;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

mod registry;

const REPO: &str = "https://github.com/clearloop/jam-test-vectors.git";
const INTO: &str = "jamtestvectors";

/// The code generator
pub struct Codegen;

impl Codegen {
    /// Run the code generator
    pub fn run() -> Result<()> {
        // get the root and output directories
        let output = PathBuf::from(env::var("OUT_DIR")?);
        let root = output.join(INTO);
        let target = output.join(INTO);

        // download the test vectors
        Self::download(&target)?;

        // write the head hash to the output directory
        let head = Self::head(&target)?;
        fs::write(output.join("head.txt"), head.trim())?;

        // generate the test vectors
        Registry::new(&root, &output).run()
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
            ])
            .status()?;
        Ok(())
    }

    fn head(target: &Path) -> Result<String> {
        let hash = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(target)
            .output()?
            .stdout;
        Ok(String::from_utf8(hash)?)
    }
}
