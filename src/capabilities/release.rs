use crate::{
    file,
    version::{
        self,
        parser::{bump_major, bump_minor, bump_patch, get_sem_version},
    },
    ReleaseArgs,
};
use crossterm::style::Stylize;
use inquire::Select;
use std::collections::HashMap;

pub fn bump(args: ReleaseArgs) {
    let version = get_sem_version();
    let ans = match args {
        ReleaseArgs {
            major: true,
            minor: _,
            patch: _,
        } => bump_major(version).to_string(),
        ReleaseArgs {
            major: _,
            minor: true,
            patch: _,
        } => bump_minor(version).to_string(),
        ReleaseArgs {
            major: _,
            minor: _,
            patch: true,
        } => bump_patch(version).to_string(),
        ReleaseArgs {
            major: _,
            minor: _,
            patch: _,
        } => user_driven_choice(version),
    };

    let _file_written = file::write::write_version_to_file(&ans);

    let _files = file::search::find_local_files();
}

fn user_driven_choice(version: version::SemanticVersion) -> String {
    let options: HashMap<String, String> = HashMap::from([
        (
            bump_major(version.clone()).to_string().green().to_string(),
            bump_major(version.clone()).to_string(),
        ),
        (
            bump_minor(version.clone()).to_string().yellow().to_string(),
            bump_minor(version.clone()).to_string(),
        ),
        (
            bump_patch(version.clone()).to_string().red().to_string(),
            bump_patch(version.clone()).to_string(),
        ),
    ]);

    let render_options = options.clone().into_keys().collect();
    let msg: String = format!("Current version is {version}");
    match Select::new(&msg, render_options).prompt() {
        Ok(answer) => options.get(&answer).unwrap().to_string(),
        Err(_) => {
            println!("Option not recognised");
            version.to_string()
        }
    }
}
