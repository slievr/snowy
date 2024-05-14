use crate::capabilities::bump;
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
    Bump {},
}

fn main() {
    let args = Cli::parse();

    let version = file::write::get_version().unwrap();

    println!("{version}");

    match args.command {
        Commands::Bump {} => bump::bump(),
    }
}
