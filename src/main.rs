use std::io;
use std::env;
use std::fs;
use std::path::PathBuf;
use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    commit: Option<String>,
}

fn find_all_files() {
    let current_dir = match env::current_dir() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Error getting current directory: {}", err);
            return;
        }
    };

    let mut matches = Vec::new();
    if let Err(err) = find_matches(&current_dir, &mut matches) {
        eprintln!("Error finding matches: {}", err);
    } else {
        for match_path in matches {
            println!("{}", match_path.display().to_string().blue());
        }
    }
}

fn find_matches(dir: &PathBuf, matches: &mut Vec<PathBuf>) -> io::Result<()> {
    let parseable_files = vec!["Cargo.toml"];
    let entries = fs::read_dir(dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            find_matches(&path, matches)?;
        } else {
            let file_name = match path.file_name().and_then(|n| n.to_str()) {
                Some(name) => name,
                None => continue, // Skip files with non-UTF-8 names
            };

            println!("{}", file_name);

            if parseable_files.contains(&file_name) {
                matches.push(path);
            }
        }
    }

    Ok(())
}

fn main() {
    let args = Args::parse();
    if let Some(commit) = args.commit {
        println!("Using Commit {}", commit)
    }

    find_all_files();

    let output_text = "Hello, world!".green();
    println!("{}", output_text);
}
