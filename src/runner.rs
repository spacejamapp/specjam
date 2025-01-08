//! The test runner

use crate::{Scale, Section, Test};
use anyhow::Result;
use colored::Colorize;
use serde_json::Value;
use std::{path::Path, process::Command};

/// A runner for test vectors
pub trait Runner {
    /// Run a test vector
    fn step(&self, test: &Test) -> Result<()>;

    /// Wrap the test process
    fn wrap_step(&self, test: &Test) -> Result<()> {
        let time = std::time::Instant::now();
        self.step(test)?;
        tracing::info!(
            "{:>8} [{:>8}] {}",
            "PASS".bright_green().bold(),
            format!("0.{:0>3}s", time.elapsed().as_millis()),
            test.to_string()
        );
        Ok(())
    }

    /// Run all test vectors
    fn process(&self, scale: Option<Scale>, sections: &[Section]) -> Result<()> {
        let time = std::time::Instant::now();
        let mut total = 0;
        let mut skipped = 0;
        let no_scale = scale.is_none();
        for section in sections {
            let tests = section.tests();
            total += tests.len();
            tracing::info!(
                "{}",
                format!(
                    "{:>8} {}",
                    "Testing".bright_green().bold(),
                    section.as_ref().purple().bold()
                )
            );

            // if the tests are not scaled, run all of them
            if tests[0].scale.is_none() {
                for test in tests {
                    self.wrap_step(test)?;
                }
                continue;
            }

            if no_scale {
                let (full, tiny): (Vec<&Test>, Vec<&Test>) =
                    tests.iter().partition(|t| t.scale == Some(Scale::Full));

                // run the tiny tests
                for test in tiny {
                    self.wrap_step(test)?;
                }

                // run the full tests
                for test in full {
                    self.wrap_step(test)?;
                }

                continue;
            }

            // if the tests are scaled, run only the tests that match the scale
            if let Some(scale) = scale {
                for test in tests {
                    if test.scale == Some(scale) {
                        self.wrap_step(test)?;
                    } else {
                        skipped += 1;
                    }
                }

                continue;
            }
        }

        tracing::info!(
            "{:>8} [{:>8}] {}",
            "SUMMARY".bright_green().bold(),
            format!("0.{:0>3}s", time.elapsed().as_millis()),
            format!(
                "{} tests run: {} tests {}, {} {}",
                format!("{total}").white().bold(),
                format!("{}", total - skipped).white().bold(),
                "passed".bright_green().bold(),
                format!("{skipped}").white().bold(),
                "skipped".yellow(),
            )
        );
        Ok(())
    }
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
pub struct BinaryRunner<'s> {
    /// The path to the binary
    path: &'s Path,
}

impl<'s> BinaryRunner<'s> {
    /// Create a new binary runner
    pub fn new(path: &'s Path) -> Self {
        Self { path }
    }
}

impl Runner for BinaryRunner<'_> {
    fn step(&self, test: &Test) -> Result<()> {
        let output = Command::new(self.path)
            .args([test.name, test.section.as_ref()])
            .output()?
            .stdout;

        let value: Value = serde_json::from_slice(&output)?;
        assert_eq!(value, test.output, "Failed to run test {test}");
        Ok(())
    }
}

/// A dummy runner for testing
pub struct DummyRunner;

impl Runner for DummyRunner {
    fn step(&self, _: &Test) -> Result<()> {
        Ok(())
    }
}
