use regex::Regex;

use super::SemanticVersion;
use crate::file;

pub fn parse_semantic_version(version_str: &str) -> Option<SemanticVersion> {
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

pub fn bump_patch(version: SemanticVersion) -> SemanticVersion {
    version.bump_patch()
}

pub fn bump_minor(version: SemanticVersion) -> SemanticVersion {
    version.bump_minor()
}

pub fn bump_major(version: SemanticVersion) -> SemanticVersion {
    version.bump_major()
}

pub fn get_sem_version() -> SemanticVersion {
    let version = file::write::get_version_from_file();
    match parse_semantic_version(&version) {
        Some(sem_version) => sem_version,
        _e => SemanticVersion::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_semantic_version() {
        let result = parse_semantic_version("v1.0.0-test").unwrap();
        let expected = SemanticVersion {
            prefix: String::from("v"),
            major: 1,
            minor: 0,
            patch: 0,
            suffix: String::from("-test"),
        };
        assert_eq!(result.prefix, expected.prefix);
        assert_eq!(result.major, expected.major);
        assert_eq!(result.minor, expected.minor);
        assert_eq!(result.patch, expected.patch);
        assert_eq!(result.suffix, expected.suffix);
    }
}
