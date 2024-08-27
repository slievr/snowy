use crossterm::style::Stylize;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

fn find_files_in_vec(root_dir: &str, files_to_match: Vec<&str>) -> io::Result<Vec<PathBuf>> {
    let mut results = Vec::new();
    let mut dirs_to_visit = vec![PathBuf::from(root_dir)];

    while let Some(dir) = dirs_to_visit.pop() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file()
                && files_to_match
                    .iter()
                    .any(|f| *f == path.file_name().unwrap())
            {
                results.push(path);
            } else if path.is_dir() {
                dirs_to_visit.push(path);
            }
        }
    }

    Ok(results)
}

pub fn find_local_files() -> io::Result<Vec<PathBuf>> {
    let current_dir = env::current_dir()?;
    let match_files = ["Cargo.toml"].to_vec();

    find_files_in_vec(current_dir.to_str().unwrap(), match_files)
}
