use std::fmt;

pub mod git;
pub mod parser;
use crossterm::style::Stylize;

#[derive(Default, Clone)]
pub struct SemanticVersion {
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

    pub fn pretty_string(&self) -> String {
        let prefix = &self.prefix.clone().dim();
        let major = &self.major.to_string().green();
        let minor = &self.minor.to_string().yellow();
        let patch = &self.patch.to_string().red();
        let suffix = &self.suffix.clone().dim();

        format!("{prefix}{major}.{minor}.{patch}{suffix}")
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
