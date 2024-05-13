use crate::parsers::file_search;
use colored::{ColoredString, Colorize};
use inquire::{InquireError, Select};

pub fn bump() {
    print!("test");
    let _files = file_search::find_local_files();
    let options: Vec<ColoredString> = vec!["Major".green(), "Minor".blue(), "Patch".red()];

    let ans: Result<ColoredString, InquireError> =
        Select::new("Select bump type", options).prompt();

    match ans {
        Ok(choice) => println!("Bumping {choice}!"),
        Err(_) => println!("There was an error, please try again"),
    }
}
