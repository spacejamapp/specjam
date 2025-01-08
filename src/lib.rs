//! The specjam library

use serde_json::Value;
use std::fmt::Display;
pub use {
    cli::App,
    runner::{BinaryRunner, Runner},
    section::Section,
};

mod cli;
mod runner;
mod section;

/// A general test vector
///
/// This is the main struct that represents a test vector
#[derive(Debug, Clone)]
pub struct Test {
    /// The scale of the test vectors
    pub scale: Scale,
    /// The section of the test vectors
    pub section: Section,
    /// The name of the test vector
    pub name: String,
    /// The input of the test vectors
    pub input: Value,
    /// The output of the test vectors
    pub output: Value,
}

/// The scale of the test vectors

#[derive(Debug, Clone, Copy)]
pub enum Scale {
    /// The test vectors are small
    Tiny,
    /// The test vectors are full
    Full,
    /// The test vectors are not available
    None,
}

impl Scale {
    pub fn to_path(&self) -> Option<String> {
        match self {
            Scale::Tiny => Some("tiny".to_string()),
            Scale::Full => Some("full".to_string()),
            Scale::None => None,
        }
    }
}

impl Display for Test {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut path = vec![];
        if let Some(scale) = self.scale.to_path() {
            path.push(scale);
        }
        path.push(self.section.to_string());
        path.push(self.name.to_string());
        write!(f, "{}", path.join("::"))
    }
}
