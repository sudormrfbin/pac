use crate::{Error, Result};

use git2::{self, Oid, Repository};
use std::{fmt::Display, fs};
use std::{path::PathBuf, str::FromStr};

/// Represents a branch, tag or commit hash
#[derive(Debug, Clone)]
pub enum GitRefKind {
    Tag,
    Branch,
    // A commit is not technically a ref; this represents the SHA-1 hash of the commit
    Commit,
}

impl Display for GitRefKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tag => write!(f, "tag"),
            Self::Branch => write!(f, "branch"),
            Self::Commit => write!(f, "commit"),
        }
    }
}

impl FromStr for GitRefKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "tag" => Ok(GitRefKind::Tag),
            "commit" => Ok(GitRefKind::Commit),
            "branch" => Ok(GitRefKind::Branch),
            _ => Err(Error::Format),
        }
    }
}

/// Represents a [GitRefKind] with it's value.
#[derive(Debug, Clone)]
pub struct GitReference {
    pub kind: GitRefKind,
    /// Tag name, branch name or commit hash.
    pub value: String,
}

impl GitReference {
    /// Make a new GitReference with [`kind`](GitRefKind) and `value`.
    /// [`Error::Format`] may be returned if `kind` is not valid.
    pub fn new(kind: &str, value: &str) -> Result<Self> {
        Ok(Self {
            kind: GitRefKind::from_str(kind)?,
            value: value.to_string(),
        })
    }

    /// Get the full name of the ref. "master" changes to "refs/heads/master", etc.
    /// For a commit returns the value (commit hash) itself.
    fn long_ref_name(&self) -> String {
        match self.kind {
            GitRefKind::Tag => format!("refs/tags/{}", self.value),
            GitRefKind::Branch => format!("refs/heads/{}", self.value),
            GitRefKind::Commit => self.value.clone(),
        }
    }
}

/// Fetch from a remote repo (branches and tags). Does not change working tree.
/// Returns the default remote branch.
fn fetch(repo: &Repository, remote: &str) -> Result<GitReference> {
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
        .ok_or(Error::Git(
            "Default branch name is invalid utf-8".to_string(),
        ))
        // s is of the form "refs/heads/master" so split and use "master" only
        .map(|s| GitReference::new("branch", s.rsplit('/').next().unwrap()))?
}

/// Fetch changes from remote for a local repo, discard changes in working tree,
/// checkout the given reference (or master if its None) and update submodules.
fn sync_repo(repo: &Repository, remote: &str, gitref: Option<GitReference>) -> Result<()> {
    let default_branch = fetch(&repo, remote)?;

    let gitref = gitref.unwrap_or(default_branch);
    let refname = gitref.long_ref_name();
    let oid = match gitref.kind {
        GitRefKind::Commit => Oid::from_str(&refname)?,
        _ => repo.refname_to_id(&refname)?,
    };

    // Object id is the SHA-1 hash of, for eg. a commit object (can be
    // any git object like a tree, blob, etc)
    let object = repo.find_object(oid, None)?;

    let mut opts = git2::build::CheckoutBuilder::new();
    opts.force(); // discard changes to working tree
    repo.checkout_tree(&object, Some(&mut opts))?;

    // set_head is needed here since checkout_tree will only change the
    // files in the working tree (HEAD will still point to a previous commit
    // and the files will appear to be staged).
    match gitref.kind {
        GitRefKind::Commit => repo.set_head_detached(oid)?,
        _ => repo.set_head(&refname)?,
    };

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
    fn clone_info(&self) -> (&str, PathBuf, Option<GitReference>);

    /// Clone a remote repository and update submodules.
    fn git_clone(&self) -> Result<()> {
        let (remote, path, reference) = self.clone_info();
        let repo = git2::Repository::init(&path)?;
        let result = sync_repo(&repo, remote, reference);
        if result.is_err() {
            fs::remove_dir_all(&path)?;
        }
        result
    }

    fn git_pull(&self) -> Result<()> {
        let (remote, path, reference) = self.clone_info();
        let repo = Repository::open(&path)?;
        sync_repo(&repo, remote, reference)
    }
}
