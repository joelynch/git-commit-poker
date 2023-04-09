use std::process::Command;

use git2::Repository;

use crate::errors::LottoError;

pub struct Commit {
    pub repo: String,
    pub hash: String,
    pub full_hash: String,
    pub date: String,
    pub summary: Option<String>,
}

impl Commit {
    pub fn new(
        repo: String,
        hash: String,
        full_hash: String,
        date: String,
        summary: Option<String>,
    ) -> Self {
        Commit {
            repo,
            hash,
            full_hash,
            date,
            summary,
        }
    }

    pub fn from_repo_and_commit(
        repo: &Repository,
        commit: git2::Commit,
    ) -> Result<Self, LottoError> {
        let repo_str = repo
            .workdir()
            .ok_or(LottoError::GitFailed)?
            .to_str()
            .ok_or(LottoError::GitFailed)?
            .to_string();
        let hash = commit
            .as_object()
            .short_id()
            .map_err(|_| LottoError::GitFailed)?
            .as_str()
            .ok_or(LottoError::GitFailed)?
            .to_string();
        let full_hash = commit.id().to_string();
        let date = commit.time().seconds().to_string();
        let summary = commit.summary().map(|s| s.to_string());
        Ok(Commit::new(repo_str, hash, full_hash, date, summary))
    }

    pub fn latest() -> Result<Self, LottoError> {
        let repo = Repository::discover(".").map_err(|_| LottoError::GitFailed)?;
        let head = repo.head().map_err(|_| LottoError::GitFailed)?;
        let commit = head.peel_to_commit().map_err(|_| LottoError::GitFailed)?;
        Self::from_repo_and_commit(&repo, commit)
    }
}

pub fn git_commit(args: Vec<String>) -> Result<(), LottoError> {
    let mut cmd = Command::new("git");
    if cmd.arg("commit").args(args).status()?.success() {
        print!("\x1B[A\x1B[2K");
    } else {
        return Err(LottoError::GitFailed);
    }
    Ok(())
}
