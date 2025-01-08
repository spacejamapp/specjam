//! The command line interface of specjam

use crate::Section;
use clap::{ArgAction, Parser, Subcommand};
use std::path::PathBuf;

/// The JAM spec test engine developed by spacejam
#[derive(Debug, Parser)]
#[clap(version, about, long_about = None)]
#[command(propagate_version = true, arg_required_else_help = true)]
pub struct App {
    /// The verbosity level
    #[clap(short, long, action = ArgAction::Count, global = true)]
    pub verbose: u8,

    /// The command of the specjam
    #[clap(subcommand)]
    command: Option<Command>,
}

/// The available commands for the specjam CLI
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Try out the dummy test runner
    Dummy,
    /// Print the input of a test
    Input {
        /// The section of the test to run
        #[clap(value_enum)]
        section: Section,
        /// The name of the test vector
        name: String,
    },
    /// List the tests of a section
    List {
        /// The section of the test to run
        #[clap(value_enum)]
        section: Section,
    },
    /// Print the output of a test
    Output {
        /// The section of the test to run
        #[clap(value_enum)]
        section: Section,
        /// The name of the test vector
        name: String,
    },
    /// Spawn a binary runner, required binary arguments:
    ///
    /// ```text
    /// USAGE:
    ///   <binary> [OPTIONS]
    ///
    /// OPTIONS:
    ///   --section <section> the name of the section
    ///   --name <name> the name of the test
    ///   --input <input> The file path of the input JSON
    /// ```
    Spawn {
        /// The path to the binary
        binary: PathBuf,
    },
    /// Prints the version of the JAM spec.
    Spec,
}
