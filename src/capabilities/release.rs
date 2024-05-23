use std::fmt;

use crate::file;
use colored::{ColoredString, Colorize};
use inquire::{InquireError, Select};
use regex::Regex;

#[derive(Default)]
struct SemanticVersion {
    prefix: String,
    major: u32,
    minor: u32,
    patch: u32,
    suffix: String,
}

impl SemanticVersion {
    fn bump_major(&self) -> SemanticVersion {
        SemanticVersion {
            prefix: self.prefix.clone(),
            major: self.major + 1,
            minor: self.minor,
            patch: self.patch,
            suffix: self.suffix.clone(),
        }
    }

    fn bump_minor(&self) -> SemanticVersion {
        SemanticVersion {
            prefix: self.prefix.clone(),
            major: self.major,
            minor: self.minor + 1,
            patch: self.patch,
            suffix: self.suffix.clone(),
        }
    }

    fn bump_patch(&self) -> SemanticVersion {
        SemanticVersion {
            prefix: self.prefix.clone(),
            major: self.major,
            minor: self.minor,
            patch: self.patch + 1,
            suffix: self.suffix.clone(),
        }
    }
}

impl fmt::Display for SemanticVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{0}{1}.{2}.{3}{4}",
            self.prefix, self.major, self.minor, self.patch, self.suffix
        )
    }
}

pub fn bump() {
    let version = file::write::get_version().unwrap();
    let _files = file::search::find_local_files();

    let sem_version: SemanticVersion = parse_semantic_version(&version).unwrap();

    let options: Vec<ColoredString> = vec![
        sem_version.bump_major().to_string().green(),
        sem_version.bump_minor().to_string().blue(),
        sem_version.bump_patch().to_string().red(),
    ];
    let msg = format!("Current version is {sem_version}");
    let ans: Result<ColoredString, InquireError> = Select::new(&msg, options).prompt();

    match ans {
        Ok(choice) => {
            println!("Bumping {choice}!");
            let _ = file::write::write_version(&choice);
        }
        Err(_) => println!("There was an error, please try again"),
    }
}

fn parse_semantic_version(version_str: &str) -> Option<SemanticVersion> {
    let re = Regex::new(
        r"^(?P<prefix>[a-zA-Z-]*)v?(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)(?P<suffix>[-+].*)?$",
    )
    .unwrap();

    re.captures(version_str).map(|caps| SemanticVersion {
        prefix: caps
            .name("prefix")
            .map_or_else(String::new, |m| m.as_str().to_string()),
        major: caps["major"].parse().unwrap(),
        minor: caps["minor"].parse().unwrap(),
        patch: caps["patch"].parse().unwrap(),
        suffix: caps
            .name("suffix")
            .map_or_else(String::new, |m| m.as_str().to_string()),
    })
}
