//! Test vector registry entry

use crate::{Scale, Section, Test};
use std::{iter::IntoIterator, path::PathBuf};

/// A test vector registry entry
pub struct Entry {
    /// The section of the test vector
    section: Section,
    /// The scale of the test vector
    scale: Scale,
    /// The directory of the test vector
    dir: PathBuf,
}
