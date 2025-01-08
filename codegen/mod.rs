//! Code generation scripts for embedding the jam test vectors into

use anyhow::Result;
use git2::{build::RepoBuilder, FetchOptions};
use registry::Registry;
use std::{env, path::PathBuf};

mod cli;
mod registry;
mod runner;

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

        // generate the test vectors
        Registry::new(&root, &output).run()
    }

    /// Download the jam test vectors
    pub fn download() -> Result<PathBuf> {
        let into = PathBuf::from(INTO);
        if into.exists() {
            return Ok(into);
        }

        let mut builder = RepoBuilder::new();
        let mut opts = FetchOptions::new();

        opts.depth(1);
        builder.fetch_options(opts);
        builder.clone(REPO, &into)?;
        Ok(into)
    }
}
