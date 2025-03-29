//! The main entry point of specjam

use clap::{ArgAction, Parser, Subcommand};
use serde_json::Value;
use specjam::{
    runner::{BinaryRunner, DummyRunner, Runner},
    Scale, Section,
};
use std::path::PathBuf;
use tracing::level_filters::LevelFilter;

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
    pub command: Command,
}

/// The available commands for the specjam CLI
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Try out the dummy test runner
    Dummy(SpawnOptions),
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
        /// The options for the spawn command
        #[clap(flatten)]
        options: SpawnOptions,
    },
}

impl Command {
    /// Run the command
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            Command::Dummy(options) => {
                let runner = DummyRunner;
                options.run(runner)?;
                Ok(())
            }
            Command::Input { section, name } => {
                let test = section
                    .tests()
                    .iter()
                    .find(|t| t.name == name)
                    .ok_or_else(|| anyhow::anyhow!("test {name} not found in {section}"))?;

                let json = serde_json::from_str::<Value>(test.input)?;
                println!("{}", serde_json::to_string_pretty(&json)?);
                Ok(())
            }
            Command::List { section } => {
                println!(
                    "{:#?}",
                    section.tests().iter().map(|t| t.name).collect::<Vec<_>>()
                );
                Ok(())
            }
            Command::Output { section, name } => {
                let test = section
                    .tests()
                    .iter()
                    .find(|t| t.name == name)
                    .ok_or_else(|| anyhow::anyhow!("test {name} not found in {section}"))?;

                let json = serde_json::from_str::<Value>(test.output)?;
                println!("{}", serde_json::to_string_pretty(&json)?);
                Ok(())
            }
            Command::Spawn { binary, options } => {
                let runner = BinaryRunner::new(binary);
                options.run(runner)?;
                Ok(())
            }
        }
    }
}

/// The options for the spawn command
#[derive(Debug, Clone, Parser)]
pub struct SpawnOptions {
    /// The scale of the test vectors, if not provided, the runner
    /// will process all scales
    #[clap(short, long)]
    pub scale: Option<Scale>,

    /// The section of the test vectors, if provided, the runner
    /// will only process the tests in the section
    #[clap(long)]
    pub section: Option<Section>,

    /// The sections to skip
    #[clap(long)]
    pub skip: Vec<Section>,
}

impl SpawnOptions {
    /// Run the tests
    pub fn run<R: Runner>(&self, runner: R) -> anyhow::Result<()> {
        let sections = if let Some(section) = self.section {
            vec![section]
        } else {
            Section::all()
                .into_iter()
                .filter(|s| !self.skip.contains(s))
                .collect()
        };

        runner.process(self.scale, &sections)
    }
}

fn main() -> anyhow::Result<()> {
    let app = App::parse();

    // Initialize the tracing subscriber
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_level(false)
        .with_max_level(match app.verbose {
            0 => LevelFilter::INFO,
            1 => LevelFilter::DEBUG,
            _ => LevelFilter::TRACE,
        })
        .init();

    app.command.run()
}
