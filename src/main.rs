use crate::capabilities::release;
use clap::{Parser, Subcommand};

mod capabilities;
mod file;

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "snowy")]
#[command(about = "A fictional versioning CLI", version, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command()]
    Release {},
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Release {} => release::bump(),
    }
}
