//! test vector registry

pub use entry::Entry;
use std::path::PathBuf;

mod entry;

/// The test vector registry
pub struct Registry {
    /// The root directory of the test vectors
    root: PathBuf,
}

impl Registry {
    /// Create a new registry from the given jam-test-vectors directory
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
}
