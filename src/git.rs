use crate::Result;

use git2::{self, Repository};
use std::fs;
use std::path::Path;

fn fetch(repo: &Repository, remote: &str) -> Result<()> {
    let mut opts = git2::FetchOptions::new();
    opts.download_tags(git2::AutotagOption::All)
        .update_fetchhead(true);

    let refspec = "refs/heads/*:refs/heads/*";
    let mut remote = repo.remote_anonymous(&remote)?;
    remote.fetch(&[refspec], Some(&mut opts), None)?;
    Ok(())
}

/// Fetch changes from remote for a local repo, hard reset HEAD
/// and update submodules.
fn sync_repo(repo: &Repository, remote: &str) -> Result<()> {
    fetch(&repo, remote)?;
    let reference = "HEAD";
    let oid = repo.refname_to_id(reference)?;
    let object = repo.find_object(oid, None)?;
    repo.reset(&object, git2::ResetType::Hard, None)?;
    update_submodules(&repo)?;
    Ok(())
}

/// Clone a remote repository and update submodules.
pub fn clone<P: AsRef<Path>>(remote: &str, path: P) -> Result<()> {
    let repo = git2::Repository::init(&path)?;
    let result = sync_repo(&repo, remote);
    if result.is_err() {
        fs::remove_dir_all(&path)?;
    }
    result
}

pub fn update<P: AsRef<Path>>(remote: &str, path: P) -> Result<()> {
    let repo = Repository::open(&path)?;
    sync_repo(&repo, remote)
}

fn update_submodules(repo: &Repository) -> Result<()> {
    fn add_subrepos(repo: &Repository, list: &mut Vec<Repository>) -> Result<()> {
        for mut subm in repo.submodules()? {
            if let Some("docs") = subm.name() {
                continue;
            }
            subm.update(true, None)?;
            list.push(subm.open()?);
        }
        Ok(())
    }

    let mut repos = Vec::new();
    add_subrepos(repo, &mut repos)?;
    while let Some(r) = repos.pop() {
        add_subrepos(&r, &mut repos)?;
    }
    Ok(())
}
