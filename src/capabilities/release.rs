use std::{fmt, io};

use crate::{file, ReleaseArgs};
use colored::{ColoredString, Colorize};
use inquire::Select;
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

pub fn bump(args: ReleaseArgs) {
    let version = file::write::get_version().unwrap();
    let sem_version: SemanticVersion = parse_semantic_version(&version).unwrap();
    let ans = match args {
        ReleaseArgs {
            major: true,
            minor: _,
            patch: _,
        } => sem_version.bump_major().to_string(),
        ReleaseArgs {
            major: _,
            minor: true,
            patch: _,
        } => sem_version.bump_major().to_string(),
        ReleaseArgs {
            major: _,
            minor: _,
            patch: true,
        } => sem_version.bump_major().to_string(),
        ReleaseArgs {
            major: _,
            minor: _,
            patch: _,
        } => user_driven_choice(sem_version),
    };

    let _file_written = file::write::write_version(&ans);

    let _files = file::search::find_local_files();
}

fn user_driven_choice(version: SemanticVersion) -> String {
    let options: Vec<ColoredString> = vec![
        version.bump_major().to_string().green(),
        version.bump_minor().to_string().blue(),
        version.bump_patch().to_string().red(),
    ];
    let msg = format!("Current version is {version}");
    Select::new(&msg, options)
        .prompt()
        .unwrap_or(version.to_string().white())
        .to_string()
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
