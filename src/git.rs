use crate::{Error, Result};

use git2::{self, Repository};
use std::fs;
use std::path::PathBuf;

/// Fetch from a remote repo (branches and tags). Does not change working tree.
/// Returns the default remote branch.
fn fetch(repo: &Repository, remote: &str) -> Result<String> {
    let mut opts = git2::FetchOptions::new();
    opts.download_tags(git2::AutotagOption::All)
        .update_fetchhead(true);

    // fetch branches and tags
    let refspec = ["refs/heads/*:refs/heads/*", "refs/tags/*:refs/tags/*"];
    let mut remote = repo.remote_anonymous(&remote)?;
    remote.fetch(&refspec, Some(&mut opts), None)?;

    remote
        .default_branch()?
        .as_str()
        .ok_or_else(|| Error::Git(
            "Default branch name is invalid utf-8".to_string(),
        ))
        // s is of the form "refs/heads/master" so split and use "master" only
        .map(|s| s.to_string())
}

/// Fetch changes from remote for a local repo, discard changes in working tree,
/// checkout the given reference (or master if its None) and update submodules.
fn sync_repo(repo: &Repository, remote: &str, refname: Option<String>) -> Result<()> {
    let default_branch = fetch(&repo, remote)?;

    let refname = refname.unwrap_or(default_branch);
    // `object` will always point to a commit disregarding intermediate
    // refs. `gitref` will be this intermediate ref, if applicable.
    let (object, gitref) = repo.revparse_ext(&refname)?;

    let mut opts = git2::build::CheckoutBuilder::new();
    opts.force(); // discard changes to working tree
    repo.checkout_tree(&object, Some(&mut opts))?;

    // set_head is needed here since checkout_tree will only change the
    // files in the working tree (HEAD will still point to a previous commit
    // and the files will appear to be staged).
    match gitref {
        Some(gref) => repo.set_head(gref.name().ok_or(Error::Format)?),
        None => repo.set_head_detached(object.id()),
    }?;

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
    /// Get (remote_url, local_path, reference) for cloning and updating repo
    fn clone_info(&self) -> (&str, PathBuf, Option<String>);

    /// Clone a remote repository and update submodules.
    fn git_clone(&self) -> Result<()> {
        let (remote, path, rev) = self.clone_info();
        let repo = git2::Repository::init(&path)?;
        let result = sync_repo(&repo, remote, rev);
        if result.is_err() {
            fs::remove_dir_all(&path)?;
        }
        result
    }

    fn git_pull(&self) -> Result<()> {
        let (remote, path, rev) = self.clone_info();
        let repo = Repository::open(&path)?;
        sync_repo(&repo, remote, rev)
    }
}
