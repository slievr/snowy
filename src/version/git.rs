use std::error::Error;

use gix::Repository;

pub fn is_in_workable_state() -> Result<bool, Box<dyn Error>> {
    if !is_base_branch()? {
        return Ok(false);
    }

    if is_changes_staged()? {
        return Ok(false);
    }

    Ok(true)
}

fn is_base_branch() -> Result<bool, Box<dyn Error>> {
    let base_branches = ["main", "master"];

    let repo = get_repo();
    let name = repo.head_name()?.unwrap();
    let shortname = name.shorten();
    let shortname_str = std::str::from_utf8(shortname)?;

    print!("[GIT] {:?}", shortname_str);

    if base_branches.contains(&shortname_str) {
        return Ok(true);
    }

    Ok(false)
}

fn is_changes_staged() -> Result<bool, Box<dyn Error>> {
    let _repo = get_repo();

    Ok(false)
}

fn get_all_refs() {
    let repo = get_repo();
    let refs = repo.references().unwrap();

    for tag_ref in refs.all().unwrap() {
        let tag_ref_res = tag_ref.unwrap();
        println!("{:?}", tag_ref_res);

        if let Some(category) = tag_ref_res.name().category() {
            if category == gix::reference::Category::Tag {
                let id = tag_ref_res.id();
                let _object = id.object().unwrap();
            }
        }
    }
}

fn get_repo() -> Repository {
    match gix::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("[GIT] failed to open: {e}"),
    }
}
