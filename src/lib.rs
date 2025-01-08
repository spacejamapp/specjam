//! The specjam library

use clap::ValueEnum;
use colored::{ColoredString, Colorize};
use std::fmt::Display;
pub use {cli::App, runner::Runner, section::Section};

mod cli;
pub mod runner;
mod section;

/// A general test vector
///
/// This is the main struct that represents a test vector
#[derive(Debug, Clone, Copy)]
pub struct Test {
    /// The scale of the test vectors
    pub scale: Option<Scale>,
    /// The section of the test vectors
    pub section: Section,
    /// The name of the test vector
    pub name: &'static str,
    /// The input of the test vectors
    pub input: &'static str,
    /// The output of the test vectors
    pub output: &'static str,
}

/// The scale of the test vectors
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
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

impl Display for Test {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut path: Vec<ColoredString> = vec![];
        if let Some(scale) = self.scale {
            path.push(scale.as_ref().to_string().bright_cyan());
        }
        path.push(self.section.to_string().bright_purple().bold());
        path.push(self.name.to_string().blue().bold());

        let len = path.len();
        let mut msg = String::new();
        for (i, patt) in path.into_iter().enumerate() {
            msg.push_str(&format!(
                "{}{}",
                patt,
                if i == len - 1 { "" } else { "::" }.dimmed()
            ));
        }
        write!(f, "{}", msg)
    }
}

pub mod registry {
    //! The registry of the test vectors
    include!(concat!(env!("OUT_DIR"), "/registry.rs"));
}
