use anyhow::{anyhow, Result};

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{exit, Command, Output};

pub fn git_exec(args: &[&str]) -> Result<Output> {
    let output = Command::new("git").args(args).output();
    match output {
        Ok(output) => Ok(output),
        Err(e) => Err(anyhow!(
            "Failed to run git. Make sure git is installed\nAdditional info: {}",
            e
        )),
    }
}

pub fn commit_as_hook(commit_message: &str) -> Result<()> {
    let output = git_exec(&["rev-parse", "--absolute-git-dir"])?;
    if !output.status.success() {
        return Err(anyhow!("Could not get git directory"));
    }
    let git_dir = PathBuf::from(String::from_utf8_lossy(&output.stdout).trim());
    let commit_file_path = git_dir.join("COMMIT_EDITMSG");
    fs::write(commit_file_path, commit_message)?;
    Ok(())
}

pub fn commit(commit_message: &str) -> Result<()> {
    let output = git_exec(&["commit", "-m", commit_message])?;
    if commit_message.chars().count() == 0 {
        return Err(anyhow!("You must enter a commit message to commit changes."));
    } 
    std::io::stdout().write_all(&output.stdout)?;
    std::io::stderr().write_all(&output.stderr)?;
    exit(
        output
            .status
            .code()
            .ok_or_else(|| anyhow!("Signal terminated"))?,
    );
}

pub fn check_staged_files() -> Result<()> {
    let output = git_exec(&["diff", "--cached", "--quiet"])?;
    if output.status.code() == Some(0) {
        return Err(anyhow!("You have not added anything please do `git add`"));
    }
    Ok(())
}