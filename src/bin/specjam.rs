//! The main entry point of specjam

use clap::Parser;
use specjam::App;
use tracing::level_filters::LevelFilter;

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
