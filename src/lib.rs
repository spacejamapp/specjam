//! The specjam library
#![doc = include_str!("../README.md")]

pub use registry::{Entry, Registry};
pub use section::{Section, Trace};

mod registry;
mod section;

/// A general test vector
///
/// This is the main struct that represents a test vector
#[derive(Debug, Clone)]
pub struct Test {
    /// The scale of the test vectors
    pub scale: Option<Scale>,
    /// The section of the test vectors
    pub section: Section,
    /// The name of the test vector
    pub name: String,
    /// The input of the test vectors
    pub input: String,
    /// The output of the test vectors
    pub output: String,
}

impl Test {
    /// Whether the test vector is full
    pub fn is_full(&self) -> bool {
        self.scale == Some(Scale::Full)
    }
}

/// The scale of the test vectors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scale {
    /// The test vectors are small
    Tiny,
    /// The test vectors are full
    Full,
}

impl AsRef<str> for Scale {
    fn as_ref(&self) -> &str {
        match self {
            Scale::Tiny => "tiny",
            Scale::Full => "full",
        }
    }
}
