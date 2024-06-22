use clap::{Args, Parser, Subcommand, ValueEnum};
use version::parser::get_sem_version;
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

fn snowy_logo() {
    // figure_head();
    let version = get_sem_version().pretty_string();
    println!(
        r#"
,__, 
(O,O)  SNOWY
( _/   {}
/_"
"#,
        version
    )
}

fn main() {
    snowy_logo();

    let args = Cli::parse();

    match args.command {
        Commands::Bootstrap(args) => bootstrap::booststrap(args),
        Commands::Release(args) => release::bump(args),
    }
}
