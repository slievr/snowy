use clap::{Args, Parser, Subcommand, ValueEnum};
use version::{git::is_in_workable_state, parser::get_sem_version};
mod commands;
mod file;
mod version;
use crate::commands::{bootstrap, release};

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

enum BootStrapEnvironment {
    #[default]
    Github,
}

#[derive(Debug, Clone, Args)]
struct BootstrapArgs {
    #[arg(value_enum)]
    env: BootStrapEnvironment,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Bootstrap(BootstrapArgs),
    Release(ReleaseArgs),
}

fn snowy_logo() {
    // figure_head();
    println!(
        r#"
,__, 
(O,O)  SNOWY
( _/   0.2.0
/_"
"#
    )
}

fn main() {
    let can_perform_operations = is_in_workable_state().unwrap_or(false);
    println!("Is workable?: {can_perform_operations}");

    snowy_logo();

    let args = Cli::parse();

    match args.command {
        Commands::Bootstrap(args) => bootstrap::booststrap(args),
        Commands::Release(args) => release::bump(args).unwrap(),
    }
}
