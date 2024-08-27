use crate::version::SemanticVersion;
use std::fs;
use toml_edit::{DocumentMut, Item};

const VERSION_KEYS: [&str; 2] = ["package", "project"];

pub fn update_toml_version(
    file_path: &str,
    new_version: &SemanticVersion,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read the TOML file
    let content = fs::read_to_string(file_path)?;
    let mut doc = content.parse::<DocumentMut>()?;

    // Update the version
    for key in VERSION_KEYS.iter() {
        if let Some(version) = doc
            .get_mut(key)
            .and_then(|section| section.as_table_mut())
            .and_then(|table| table.get_mut("version"))
        {
            *version = Item::Value(new_version.to_string().into());
            // Write the updated TOML back to the file
            fs::write(file_path, doc.to_string())?;
            println!("File: {file_path} updated to version: {new_version}");

            return Ok(());
        }
    }

    Err("Version field not found in TOML file".into())
}

pub fn read_toml_version(file_path: &str) -> Result<SemanticVersion, Box<dyn std::error::Error>> {
    // Read the TOML file
    let content = fs::read_to_string(file_path)?;
    let doc = content.parse::<DocumentMut>()?;

    // Extract the version
    for key in VERSION_KEYS.iter() {
        if let Some(version_str) = doc
            .get(key)
            .and_then(|section| section.as_table())
            .and_then(|table| table.get("version"))
            .and_then(|v| v.as_str())
        {
            // Parse the version string into a SemanticVersion
            return crate::version::parser::parse_semantic_version(version_str)
                .ok_or("Unable to parse version".into());
        }
    }

    Err("Version field not found in TOML file".into())
}
