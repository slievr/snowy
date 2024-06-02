use crate::capabilities::release;
use clap::{Args, Parser, Subcommand};
use figlet_rs::FIGfont;

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

#[derive(Debug, Clone, Args)]
struct ReleaseArgs {
    /// auto inc major
    #[arg(long)]
    major: bool,

    /// auto inc minor
    #[arg(long)]
    minor: bool,

    /// auto inc patch
    #[arg(long)]
    patch: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Release(ReleaseArgs),
}

fn main() {
    let args = Cli::parse();

    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("snowy").unwrap();
    println!("{}", figure);

    match args.command {
        Commands::Release(args) => release::bump(args),
    }
}
