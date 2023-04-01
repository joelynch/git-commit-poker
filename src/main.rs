mod errors;
mod lotto;

use crate::{errors::LottoError, lotto::matching_rules};
use std::{env, process::Command};

fn git_commit(args: Vec<String>) -> Result<(), LottoError> {
    let mut cmd = Command::new("git");
    if !cmd.arg("commit").args(args).status()?.success() {
        return Err(LottoError::GitFailed);
    }
    Ok(())
}

fn last_commit_hash() -> Result<String, LottoError> {
    let mut cmd = Command::new("git");
    let output = cmd.arg("rev-parse").arg("--short").arg("HEAD").output()?;
    if !output.status.success() {
        return Err(LottoError::GitFailed);
    }
    let hash = String::from_utf8(output.stdout)
        .expect("Could not parse git hash")
        .trim()
        .into();
    Ok(hash)
}

fn main() -> Result<(), LottoError> {
    let commit_args: Vec<String> = env::args().skip(1).collect();
    git_commit(commit_args)?;
    let hash = last_commit_hash()?;
    println!("Committed with hash {}", hash);
    matching_rules(&hash).into_iter().for_each(|rule| {
        println!(
            "{}: {}.  You get {} points for this one.  Nice!",
            rule.name(),
            rule.description(),
            rule.points(),
        )
    });
    Ok(())
}
