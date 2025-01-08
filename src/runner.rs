//! The test runner

use crate::Test;
use anyhow::Result;
use serde_json::Value;
use std::{path::PathBuf, process::Command};

/// A runner for test vectors
pub trait Runner {
    /// Run a test vector
    fn run(&self, test: Test) -> Result<()>;
}

/// binary test runner, which requires the following CLI arguments:
///
/// ```text
/// USAGE:
///   <binary> [OPTIONS]
///
/// Options:
///   --section <section> the name of the section
///   --name <name> the name of the test
///   --input <input> The file path of the input JSON
/// ```
#[derive(Debug, Clone)]
pub struct BinaryRunner {
    /// The path to the binary
    path: PathBuf,
}

impl Runner for BinaryRunner {
    fn run(&self, test: Test) -> Result<()> {
        let output = Command::new(&self.path)
            .args(&[&test.name, test.section.as_ref()])
            .output()?
            .stdout;

        let value: Value = serde_json::from_slice(&output)?;
        assert_eq!(value, test.output, "Failed to run test {test}");
        Ok(())
    }
}
