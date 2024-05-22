use crate::file;
use colored::{ColoredString, Colorize};
use inquire::{InquireError, Select};

pub fn bump() {
    let version = file::write::get_version().unwrap();
    let _files = file::search::find_local_files();

    let options: Vec<ColoredString> = vec![
        bump_major(&version).green(),
        bump_minor(&version).blue(),
        bump_patch(&version).red(),
    ];
    let msg = format!("Current version is {version}");
    let ans: Result<ColoredString, InquireError> = Select::new(&msg, options).prompt();

    match ans {
        Ok(choice) => {
            println!("Bumping {choice}!");
            let _ = file::write::write_version(&choice);
        }
        Err(_) => println!("There was an error, please try again"),
    }
}

fn bump_major(version: &str) -> String {
    get_bump_str(version, 0)
}

fn bump_minor(version: &str) -> String {
    get_bump_str(version, 1)
}

fn bump_patch(version: &str) -> String {
    get_bump_str(version, 2)
}

fn get_bump_str(version: &str, bump_type: usize) -> String {
    let mut version_parts = version
        .split('.')
        .map(|part| part.to_string())
        .collect::<Vec<String>>();
    let major = version_parts[bump_type].parse::<u32>().unwrap() + 1;
    version_parts[bump_type] = major.to_string();

    return version_parts
        .iter()
        .map(|part| part.to_string())
        .collect::<Vec<String>>()
        .join(".");
}
