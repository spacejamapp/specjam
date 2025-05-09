//! The section of the test vectors

use std::{fmt::Display, str::FromStr};

/// A section of the test vectors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum Section {
    /// The accumulate section
    Accumulate,
    /// The assurances section
    Assurances,
    /// The authorizations section
    Authorizations,
    /// The codec section
    Codec,
    /// The disputes section
    Disputes,
    /// The history section
    History,
    /// The preimages section
    Preimages,
    /// The pvm section
    Pvm,
    /// The reports section
    Reports,
    /// The safrole section
    Safrole,
    /// The statistics section
    Statistics,
    /// The shuffle section
    Shuffle,
    /// State trace section
    Trace,
    /// The trie section
    Trie,
}

impl Section {
    /// The all sections
    pub fn all() -> [Section; 14] {
        [
            Section::Accumulate,
            Section::Assurances,
            Section::Codec,
            Section::Pvm,
            Section::Safrole,
            Section::Statistics,
            Section::Authorizations,
            Section::Disputes,
            Section::History,
            Section::Preimages,
            Section::Reports,
            Section::Shuffle,
            Section::Trie,
            Section::Trace,
        ]
    }
}

impl FromStr for Section {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "accumulate" => Ok(Section::Accumulate),
            "assurances" => Ok(Section::Assurances),
            "codec" => Ok(Section::Codec),
            "pvm" => Ok(Section::Pvm),
            "safrole" => Ok(Section::Safrole),
            "statistics" => Ok(Section::Statistics),
            "authorizations" => Ok(Section::Authorizations),
            "disputes" => Ok(Section::Disputes),
            "history" => Ok(Section::History),
            "preimages" => Ok(Section::Preimages),
            "reports" => Ok(Section::Reports),
            "shuffle" => Ok(Section::Shuffle),
            "trace" => Ok(Section::Trace),
            "trie" => Ok(Section::Trie),
            _ => Err(anyhow::anyhow!("Invalid section {s}")),
        }
    }
}

impl AsRef<str> for Section {
    fn as_ref(&self) -> &str {
        match self {
            Section::Accumulate => "accumulate",
            Section::Assurances => "assurances",
            Section::Codec => "codec",
            Section::Pvm => "pvm",
            Section::Safrole => "safrole",
            Section::Statistics => "statistics",
            Section::Authorizations => "authorizations",
            Section::Disputes => "disputes",
            Section::History => "history",
            Section::Preimages => "preimages",
            Section::Reports => "reports",
            Section::Shuffle => "shuffle",
            Section::Trace => "trace",
            Section::Trie => "trie",
        }
    }
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
