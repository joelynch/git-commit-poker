use std::path::PathBuf;

use chrono::{Local, LocalResult, TimeZone, Utc};
use clap::Parser;
use commit_poker::{
    errors::LottoError,
    highscores::{HighScores, HighScoresImpl, ScoreInfo},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
enum Cli {
    Highscores {
        #[clap(short, default_value = "10")]
        n: usize,
        #[clap(short, long)]
        repo: Option<PathBuf>,
    },
}

fn main() -> Result<(), LottoError> {
    let args = Cli::parse();
    match args {
        Cli::Highscores { n, repo } => {
            HighScoresImpl::standard()?
                .load(repo.as_deref())?
                .into_iter()
                .take(n)
                .for_each(|s| println!("{}", format_score(&s)));
        }
    }
    Ok(())
}

fn format_score(score: &ScoreInfo) -> String {
    let rules = score
        .rules
        .iter()
        .map(|r| r.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    let datetime = match Local.timestamp_opt(score.date, 0) {
        LocalResult::Single(dt) => dt.format("%Y-%m-%d %H:%M:%S %z"),
        LocalResult::Ambiguous(dt, _) => dt.format("%Y-%m-%d %H:%M:%S %z"),
        LocalResult::None => Utc
            .timestamp_opt(score.date, 0)
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %z"),
    };
    format!(
        "{}: {} points ({}) scored on {}",
        score.commit, score.score, rules, datetime
    )
}
