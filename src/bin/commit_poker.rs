use commit_poker::git::{git_commit, Commit};
use commit_poker::highscores::{HighScores, HighScoresImpl, ScoreInfo};
use commit_poker::lotto::LottoResult;
use commit_poker::output::{TerminalOutputer, TerminalOutputerImpl};

use commit_poker::errors::LottoError;
use std::env;

fn main() -> Result<(), LottoError> {
    let commit_args: Vec<String> = env::args().skip(1).collect();
    let output = TerminalOutputerImpl::new();
    let highscores = HighScoresImpl::default()?;
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
    let scores = highscores.load(Some(&commit.repo))?;
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
