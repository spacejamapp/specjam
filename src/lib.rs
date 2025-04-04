//! The specjam library
//!
//! Current test vector version: 0.6.4

pub use section::Section;

/// Registry of the test vectors
#[allow(clippy::all)]
#[rustfmt::skip]
pub mod registry;
#[cfg(feature = "runner")]
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

impl Test {
    /// Whether the test vector is full
    pub fn is_full(&self) -> bool {
        self.scale == Some(Scale::Full)
    }
}

/// The scale of the test vectors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
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

#[cfg(feature = "runner")]
mod display {
    use crate::Test;
    use colored::{ColoredString, Colorize};

    impl std::fmt::Display for Test {
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
}
