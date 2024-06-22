use clap::{Args, Parser, Subcommand, ValueEnum};
use figlet_rs::FIGfont;
mod capabilities;
mod file;
mod version;
use crate::capabilities::{bootstrap, release};

#[derive(Debug, Parser)]
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

#[derive(Debug, Clone, ValueEnum, Default)]

enum BootStrapEnviornment {
    #[default]
    Github,
}

#[derive(Debug, Clone, Args)]
struct BootstrapArgs {
    #[arg(value_enum)]
    env: BootStrapEnviornment,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Bootstrap(BootstrapArgs),
    Release(ReleaseArgs),
}

fn figure_head() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("snowy").unwrap();
    println!("{}", figure);
}

fn main() {
    figure_head();

    let args = Cli::parse();

    match args.command {
        Commands::Bootstrap(args) => bootstrap::booststrap(args),
        Commands::Release(args) => release::bump(args),
    }
}
