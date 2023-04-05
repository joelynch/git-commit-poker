mod errors;
mod lotto;
mod output;

use lotto::LottoResult;
use output::{TerminalOutputer, TerminalOutputerImpl};

use errors::LottoError;
use std::{env, process::Command};

fn git_commit(args: Vec<String>) -> Result<(), LottoError> {
    let mut cmd = Command::new("git");
    if cmd.arg("commit").args(args).status()?.success() {
        print!("\x1B[A\x1B[2K");
    } else {
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

fn commit_lotto(output: impl TerminalOutputer, commit_args: Vec<String>) -> Result<(), LottoError> {
    output.pre_commit();
    git_commit(commit_args)?;
    let hash = last_commit_hash()?;
    let results = LottoResult::new(&hash);
    output.post_commit(&results);
    Ok(())
}

fn main() -> Result<(), LottoError> {
    let commit_args: Vec<String> = env::args().skip(1).collect();
    let output = TerminalOutputerImpl::new();
    commit_lotto(output, commit_args);
    Ok(())
}
