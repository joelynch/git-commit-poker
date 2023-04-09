use std::{
    fs::{File, OpenOptions},
    io::{Seek, Write},
    path::Path,
};

use anyhow::anyhow;
use directories::ProjectDirs;
use fs4::FileExt;
use serde::{Deserialize, Serialize};

use crate::{errors::LottoError, git::Commit, lotto::LottoResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreInfo {
    pub repo: String,
    pub commit: String,
    pub score: u64,
    pub date: String,
    pub rules: Vec<String>,
}

impl ScoreInfo {
    pub fn new(result: &LottoResult, commit: &Commit) -> Self {
        ScoreInfo {
            repo: commit.repo.clone(),
            commit: commit.hash.clone(),
            score: result.total_points(),
            date: commit.date.clone(),
            rules: result.rules.iter().map(|r| r.name()).collect(),
        }
    }
}

pub trait HighScores {
    fn save(&mut self, result: ScoreInfo) -> Result<(), LottoError>;
    fn load(&mut self, repo: &str) -> Result<Vec<ScoreInfo>, LottoError>;
}

pub struct HighScoresImpl {
    file: File,
}

impl HighScores for HighScoresImpl {
    fn save(&mut self, result: ScoreInfo) -> Result<(), LottoError> {
        self.save_innner(result)
            .map_err(LottoError::ApplicationDirError)
    }

    fn load(&mut self, repo: &str) -> Result<Vec<ScoreInfo>, LottoError> {
        self.load_inner(repo)
            .map_err(LottoError::ApplicationDirError)
    }
}

impl HighScoresImpl {
    pub fn new(path: &Path) -> Result<Self, LottoError> {
        let parent = path
            .parent()
            .ok_or(LottoError::ApplicationDirError(anyhow!(
                "could not get parent of path"
            )))?;
        std::fs::create_dir_all(parent)?;
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .unwrap();
        Ok(HighScoresImpl { file })
    }

    pub fn default() -> Result<Self, LottoError> {
        let data_dir = ProjectDirs::from("com", "joelynch", "CommitLotto").ok_or(
            LottoError::ApplicationDirError(anyhow!("could not get data dir")),
        )?;
        let path = data_dir.data_local_dir().join("highscores.json");
        HighScoresImpl::new(&path)
    }

    fn load_inner(&mut self, repo: &str) -> anyhow::Result<Vec<ScoreInfo>> {
        self.file.lock_shared()?;
        let mut scores = self
            .scores()?
            .into_iter()
            .filter(|s| repo == s.repo)
            .collect::<Vec<_>>();
        self.file.unlock()?;
        scores.sort_by_key(|s| -(s.score as i64));
        Ok(scores)
    }

    fn save_innner(&mut self, result: ScoreInfo) -> anyhow::Result<()> {
        self.file.lock_exclusive()?;
        let mut scores = self.scores()?;
        scores.push(result);
        self.file.set_len(0)?;
        self.file.rewind()?;
        serde_json::to_writer(&self.file, &scores)?;
        self.file.flush()?;
        self.file.unlock()?;
        Ok(())
    }

    fn scores(&mut self) -> anyhow::Result<Vec<ScoreInfo>> {
        if self.file.metadata()?.len() == 0 {
            return Ok(vec![]);
        }
        self.file.rewind()?;
        serde_json::from_reader(&self.file).map_err(Into::into)
    }
}