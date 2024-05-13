use crate::cli::bump;
use clap::{Command, Parser, Subcommand, ValueEnum};

mod cli;
mod parsers;

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
    Bump {},
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Bump {} => bump::bump(),
    }
}
