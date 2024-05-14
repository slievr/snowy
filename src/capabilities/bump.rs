use crate::file;
use colored::{ColoredString, Colorize};
use inquire::{InquireError, Select};

pub fn bump() {
    let version = file::write::get_version().unwrap();
    let _files = file::search::find_local_files();
    let options: Vec<ColoredString> = vec!["Major".green(), "Minor".blue(), "Patch".red()];
    let msg = format!("Current version is {version}");
    let ans: Result<ColoredString, InquireError> = Select::new(&msg, options).prompt();

    match ans {
        Ok(choice) => println!("Bumping {choice}!"),
        Err(_) => println!("There was an error, please try again"),
    }
}
