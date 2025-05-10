//! test vector registry

use crate::{section::Trace, Scale, Section};
use anyhow::Result;
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
    pub fn new(root: impl Into<PathBuf>) -> Self {
        let root = root.into();
        if !root.exists() {
            panic!(
                "jam-test-vectors directory does not exist: {}",
                root.display()
            );
        }
        Self { root }
    }

    /// Get an entry from the registry
    pub fn entry(&self, section: &str) -> Result<Entry> {
        match section {
            "accumulate" => self.accumulate(Scale::Tiny),
            "assurances" => self.assurances(Scale::Tiny),
            "authorizations" => self.authorizations(Scale::Tiny),
            "codec" => self.codec(),
            "disputes" => self.disputes(Scale::Tiny),
            "history/data" => self.history(),
            "preimages/data" => self.preimages(),
            "pvm/programs" => self.pvm(),
            "reports" => self.reports(Scale::Tiny),
            "safrole" => self.safrole(Scale::Tiny),
            "statistics" => self.statistics(Scale::Tiny),
            "shuffle" => self.shuffle(),
            "trace/fallback" => self.trace(Trace::Fallback),
            "trace/safrole" => self.trace(Trace::Safrole),
            "trace/reports-l0" => self.trace(Trace::ReportsL0),
            "trie" => self.trie(),
            _ => Err(anyhow::anyhow!("invalid section: {}", section)),
        }
    }

    /// Get the accumulate test vectors
    pub fn accumulate(&self, scale: Scale) -> Result<Entry> {
        let entry = Entry::new(Section::Accumulate, Some(scale), &self.root)?;
        Ok(entry)
    }

    /// Get the assurances test vectors
    pub fn assurances(&self, scale: Scale) -> Result<Entry> {
        let entry = Entry::new(Section::Assurances, Some(scale), &self.root)?;
        Ok(entry)
    }

    /// Get the authorizations test vectors
    pub fn authorizations(&self, scale: Scale) -> Result<Entry> {
        let entry = Entry::new(Section::Authorizations, Some(scale), &self.root)?;
        Ok(entry)
    }

    /// Get the codec test vectors
    pub fn codec(&self) -> Result<Entry> {
        let entry = Entry::new(Section::Codec, None, &self.root)?;
        Ok(entry)
    }

    /// Get the disputes test vectors
    pub fn disputes(&self, scale: Scale) -> Result<Entry> {
        let entry = Entry::new(Section::Disputes, Some(scale), &self.root)?;
        Ok(entry)
    }

    /// Get the history test vectors
    pub fn history(&self) -> Result<Entry> {
        let entry = Entry::new(Section::History, None, &self.root)?;
        Ok(entry)
    }

    /// Get the preimages test vectors
    pub fn preimages(&self) -> Result<Entry> {
        let entry = Entry::new(Section::Preimages, None, &self.root)?;
        Ok(entry)
    }

    /// Get the pvm test vectors
    pub fn pvm(&self) -> Result<Entry> {
        let entry = Entry::new(Section::Pvm, None, &self.root)?;
        Ok(entry)
    }

    /// Get the reports test vectors
    pub fn reports(&self, scale: Scale) -> Result<Entry> {
        let entry = Entry::new(Section::Reports, Some(scale), &self.root)?;
        Ok(entry)
    }

    /// Get the safrole test vectors
    pub fn safrole(&self, scale: Scale) -> Result<Entry> {
        let entry = Entry::new(Section::Safrole, Some(scale), &self.root)?;
        Ok(entry)
    }

    /// Get the statistics test vectors
    pub fn statistics(&self, scale: Scale) -> Result<Entry> {
        let entry = Entry::new(Section::Statistics, Some(scale), &self.root)?;
        Ok(entry)
    }

    /// Get the shuffle test vectors
    pub fn shuffle(&self) -> Result<Entry> {
        let entry = Entry::new(Section::Shuffle, None, &self.root)?;
        Ok(entry)
    }

    /// Get the trace test vectors
    pub fn trace(&self, trace: Trace) -> Result<Entry> {
        let entry = Entry::new(Section::Trace(trace), None, &self.root)?;
        Ok(entry)
    }

    /// Get the trie test vectors
    pub fn trie(&self) -> Result<Entry> {
        let entry = Entry::new(Section::Trie, None, &self.root)?;
        Ok(entry)
    }
}
