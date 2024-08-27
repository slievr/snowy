use crate::{
    file,
    version::{
        self,
        parser::{bump_major, bump_minor, bump_patch, get_sem_version},
        toml_edit::update_toml_version,
        SemanticVersion,
    },
    ReleaseArgs,
};
use crossterm::style::Stylize;
use inquire::Select;
use std::collections::HashMap;

pub fn bump(args: ReleaseArgs) -> Result<(), Box<dyn std::error::Error>> {
    let version = get_sem_version();
    let ans = match args {
        ReleaseArgs {
            major: true,
            minor: _,
            patch: _,
        } => bump_major(version),
        ReleaseArgs {
            major: _,
            minor: true,
            patch: _,
        } => bump_minor(version),
        ReleaseArgs {
            major: _,
            minor: _,
            patch: true,
        } => bump_patch(version),
        ReleaseArgs {
            major: _,
            minor: _,
            patch: _,
        } => user_driven_choice(version),
    };

    file::write::write_version_to_file(&ans.to_string())?;
    let files = file::search::find_local_files()?;
    for file in files {
        update_toml_version(file.to_str().unwrap(), &ans)?;
    }
    Ok(())
}

fn user_driven_choice(version: version::SemanticVersion) -> SemanticVersion {
    let options: HashMap<String, SemanticVersion> = HashMap::from([
        (
            bump_major(version.clone()).to_string().green().to_string(),
            bump_major(version.clone()),
        ),
        (
            bump_minor(version.clone()).to_string().yellow().to_string(),
            bump_minor(version.clone()),
        ),
        (
            bump_patch(version.clone()).to_string().red().to_string(),
            bump_patch(version.clone()),
        ),
    ]);

    let render_options = options.clone().into_keys().collect();
    let msg: String = format!("Current version is {version}");
    match Select::new(&msg, render_options).prompt() {
        Ok(answer) => options.get(&answer).unwrap().clone(),
        Err(_) => {
            println!("Option not recognised");
            version
        }
    }
}
