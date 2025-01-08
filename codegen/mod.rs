//! Code generation scripts for embedding the jam test vectors into

use anyhow::Result;
use git2::{build::RepoBuilder, FetchOptions};
use std::path::PathBuf;

const REPO: &str = "https://github.com/clearloop/jam-test-vectors.git";
const INTO: &str = "jamtestvectors";

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
