//! Test vector registry entry

use crate::{Scale, Section, Test};
use anyhow::Result;
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// A test vector registry entry
pub struct Entry {
    /// The section of the test vector
    section: Section,

    /// The scale of the test vector
    scale: Option<Scale>,

    /// The directory of the test vector
    files: Vec<PathBuf>,

    /// The current index of the test vector
    current: usize,
}

impl Entry {
    /// Create a new test vector registry entry
    pub fn new(section: Section, scale: Option<Scale>, stf: &Path) -> Result<Self> {
        let dir = {
            if let Some(scale) = scale {
                stf.join(section.as_ref()).join(scale.as_ref())
            } else {
                stf.join(section.as_ref())
            }
        };

        if !dir.exists() {
            return Err(anyhow::anyhow!("directory does not exist"));
        }

        let mut files = Vec::new();
        for entry in fs::read_dir(stf)? {
            let path = entry?.path();
            if path.is_file() && path.extension().unwrap_or_default() == "json" {
                files.push(path);
            }
        }

        Ok(Self {
            section,
            scale,
            files,
            current: 0,
        })
    }

    /// Get the number of test vectors
    pub fn count(&self) -> usize {
        self.files.len()
    }

    /// Get a test vector by index
    pub fn get(&self, index: usize) -> Result<Test> {
        let path = self
            .files
            .get(index)
            .ok_or_else(|| anyhow::anyhow!("index out of bounds"))?;
        self.parse(path)
    }

    /// Parse a test vector from a file
    pub fn parse(&self, path: &PathBuf) -> Result<Test> {
        match self.section {
            Section::Accumulate => self.parse_general(path),
            Section::Assurances => self.parse_general(path),
            Section::Authorizations => self.parse_general(path),
            Section::Codec => self.parse_codec(path),
            Section::Pvm => self.parse_pvm(path),
            Section::Shuffle => self.parse_shuffle(path),
            Section::Trie => self.parse_trie(path),
            Section::Trace(_) => self.parse_trace(path),
            Section::Reports => self.parse_general(path),
            Section::Statistics => self.parse_general(path),
            Section::Safrole => self.parse_general(path),
            Section::Disputes => self.parse_general(path),
            Section::History => self.parse_general(path),
            Section::Preimages => self.parse_general(path),
        }
    }

    /// Parse a codec test vector from a file
    fn parse_codec(&self, path: &PathBuf) -> Result<Test> {
        let name = Self::file_name(path)?;
        let input = hex::encode(fs::read(path.with_extension("bin"))?);
        let output = fs::read_to_string(path)?;
        let input = serde_json::json!({
            "input": input,
        })
        .to_string();

        let output = serde_json::json!({
            "output": output,
        })
        .to_string();

        Ok(Test {
            input,
            output,
            name,
            scale: self.scale,
            section: self.section,
        })
    }

    /// Parse a scaled test vector from a file
    fn parse_general(&self, path: &PathBuf) -> Result<Test> {
        let name = Self::file_name(path)?;
        let json: Value = serde_json::from_slice(&fs::read(path)?)?;
        let input = serde_json::json!({
            "input": json["input"],
            "pre_state": json["pre_state"],
        })
        .to_string();

        let output = serde_json::json!({
            "output": json["output"],
            "post_state": json["post_state"],
        })
        .to_string();

        Ok(Test {
            input,
            output,
            name,
            scale: self.scale,
            section: self.section,
        })
    }

    /// Parse a pvm test vector from a file
    fn parse_pvm(&self, path: &PathBuf) -> Result<Test> {
        let name = Self::file_name(path)?;
        let json: Value = serde_json::from_slice(&fs::read(path)?)?;

        let input = serde_json::json!({
            "name": json["name"],
            "initial-regs": json["pre-state"],
            "initial-pc": json["initial-pc"],
            "initial-regs": json["initial-regs"],
            "initial-page-map": json["initial-page-map"],
            "initial-memory": json["initial-memory"],
            "initial-gas": json["initial-gas"],
            "program": json["program"],

        })
        .to_string();

        let output = serde_json::json!({
            "expected-status": json["expected-status"],
            "expected-regs": json["expected-regs"],
            "expected-pc": json["expected-pc"],
            "expected-memory": json["expected-memory"],
            "expected-gas": json["expected-gas"],
        })
        .to_string();

        Ok(Test {
            input,
            output,
            scale: self.scale,
            section: self.section,
            name,
        })
    }

    /// Parse a trace test vector from a file
    fn parse_trace(&self, path: &PathBuf) -> Result<Test> {
        let name = Self::file_name(path)?;
        let json: Value = serde_json::from_slice(&fs::read(path)?)?;
        let input = serde_json::json!({
            "block": json["block"],
            "pre_state": json["pre_state"],
        })
        .to_string();

        let output = serde_json::json!({
            "post_state": json["post_state"],
        })
        .to_string();

        Ok(Test {
            input,
            output,
            scale: self.scale,
            section: self.section,
            name,
        })
    }

    fn parse_trie(&self, path: &PathBuf) -> Result<Test> {
        let name = Self::file_name(path)?;
        let json: Value = serde_json::from_slice(&fs::read(path)?)?;
        let input = serde_json::json!({
            "input": json["input"],
        })
        .to_string();

        let output = serde_json::json!({
            "output": json["output"],
        })
        .to_string();

        Ok(Test {
            input,
            output,
            scale: self.scale,
            section: self.section,
            name,
        })
    }

    fn parse_shuffle(&self, path: &PathBuf) -> Result<Test> {
        let name = Self::file_name(path)?;
        let json: Value = serde_json::from_slice(&fs::read(path)?)?;
        let input = serde_json::json!({
            "input": json["input"],
            "entropy": json["entropy"],
        })
        .to_string();

        let output = serde_json::json!({
            "output": json["output"],
        })
        .to_string();

        Ok(Test {
            input,
            output,
            scale: self.scale,
            section: self.section,
            name,
        })
    }

    fn file_name(path: &Path) -> Result<String> {
        Ok(path
            .with_extension("")
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("invalid file name"))?
            .to_string_lossy()
            .to_string()
            .replace('-', "_"))
    }
}

impl Iterator for Entry {
    type Item = Test;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(path) = self.files.get(self.current) else {
            return None;
        };

        let test = self.parse(path).ok()?;
        self.current += 1;
        Some(test)
    }
}
