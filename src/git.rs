use crate::{Error, Result};

use git2::{self, Repository};
use std::fs;
use std::path::PathBuf;

/// Fetch remote branches to local repo and return the default branch.
fn fetch(repo: &Repository, remote: &str) -> Result<String> {
    let mut opts = git2::FetchOptions::new();
    opts.download_tags(git2::AutotagOption::All)
        .update_fetchhead(true);

    let refspec = "refs/heads/*:refs/heads/*";
    let mut remote = repo.remote_anonymous(&remote)?;
    remote.fetch(&[refspec], Some(&mut opts), None)?;

    remote
        .default_branch()?
        .as_str()
        .ok_or(Error::Git(
            "Default branch name is invalid utf-8".to_string(),
        ))
        .map(|s| s.to_string())
}

/// Fetch changes from remote for a local repo, hard reset HEAD
/// and update submodules.
fn sync_repo(repo: &Repository, remote: &str) -> Result<()> {
    let branch_ref = fetch(&repo, remote)?;
    let oid = repo.refname_to_id(&branch_ref)?;
    let object = repo.find_object(oid, None)?;
    repo.reset(&object, git2::ResetType::Hard, None)?;
    update_submodules(&repo)?;
    Ok(())
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

/// Trait representing high level git operations on a repo
pub trait GitRepo {

    /// Get (remote_url, local_path) for cloning and updating repo
    fn path_info(&self) -> (&str, PathBuf);

    /// Clone a remote repository and update submodules.
    fn git_clone(&self) -> Result<()> {
        let (remote, path) = self.path_info();
        let repo = git2::Repository::init(&path)?;
        let result = sync_repo(&repo, remote);
        if result.is_err() {
            fs::remove_dir_all(&path)?;
        }
        result
    }

    fn git_pull(&self) -> Result<()> {
        let (remote, path) = self.path_info();
        let repo = Repository::open(&path)?;
        sync_repo(&repo, remote)
    }
}
