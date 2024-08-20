use gix::{progress::prodash::progress, refs::transaction::PreviousValue, Repository};
use std::{error::Error, process::Command};

pub fn is_in_workable_state() -> Result<bool, Box<dyn Error>> {
    let repo = get_repo()?;

    let head = repo.head()?;

    // TODO
    // Check if the current branch is "main" or "master"
    let branch_name = head.name().shorten();
    println!("{}", branch_name);
    if branch_name != "refs/heads/main" && branch_name != "refs/heads/master" {
        return Ok(false);
    }

    // Check for staged and unstaged changes
    let _status = repo.status(progress::Discard);

    Ok(true)
}

fn get_repo() -> Result<Repository, String> {
    gix::open(".").map_err(|e| format!("Failed to open repository: {}", e))
}

fn tag_repo_stupid(tag_name: &str) -> Result<(), String> {
    let output = Command::new("git")
        .args(["tag", tag_name])
        .output()
        .map_err(|e| format!("Failed to execute git tag: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Git tag failed: {}", stderr))
    }
}

fn tag_repo(tag_name: &str, message: String) -> Result<(), String> {
    let repo = gix::open(".").map_err(|e| format!("Failed to open repository: {}", e))?;

    let head_commit = repo
        .head()
        .map_err(|e| format!("Unable to get repo head: {}", e))?
        .peel_to_commit_in_place()
        .map_err(|e| format!("Failed to get head commit: {}", e))?;

    let tagger: Option<gix::actor::SignatureRef> = None;

    repo.tag(
        tag_name,
        head_commit.id(),
        gix::object::Kind::Commit,
        tagger,
        message,
        PreviousValue::Any,
    )
    .map_err(|e| format!("Failed to create tag: {}", e))?;

    Ok(())
}
