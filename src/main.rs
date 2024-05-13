use clap::Parser;
mod file_search;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    commit: Option<String>,
}

fn main() {
    let args = Args::parse();
    if let Some(commit) = args.commit {
        println!("Using Commit {}", commit)
    }

    let _files = file_search::find_local_files();
}
