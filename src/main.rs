mod errors;
mod git;
mod highscores;
mod lotto;
mod output;

use git::{git_commit, Commit};
use highscores::{HighScores, ScoreInfo};
use lotto::LottoResult;
use output::{TerminalOutputer, TerminalOutputerImpl};

use errors::LottoError;
use std::env;

fn main() -> Result<(), LottoError> {
    let commit_args: Vec<String> = env::args().skip(1).collect();
    let output = TerminalOutputerImpl::new();
    let highscores = highscores::HighScoresImpl::default()?;
    commit_lotto(output, highscores, commit_args)
}

fn commit_lotto(
    output: impl TerminalOutputer,
    mut highscores: impl HighScores,
    commit_args: Vec<String>,
) -> Result<(), LottoError> {
    output.pre_commit();
    git_commit(commit_args)?;
    let commit = Commit::latest()?;
    let results = LottoResult::new(&commit.hash);
    let scores = highscores.load(&commit.repo)?;
    let score = ScoreInfo::new(&results, &commit);
    output.post_commit(&results);
    if let Some(old_score) = scores.first() {
        if score.score > old_score.score {
            output.high_score(&score, old_score);
        }
    }
    highscores.save(score)?;
    Ok(())
}
