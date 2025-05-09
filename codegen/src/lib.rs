//! Code generation scripts for embedding the jam test vectors into

use anyhow::Result;
pub use registry::Registry;
use std::path::Path;

mod registry;

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
