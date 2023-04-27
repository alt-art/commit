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

pub fn get_git_path() -> Result<PathBuf> {
    let output = git_exec(&["rev-parse", "--absolute-git-dir"])?;
    if !output.status.success() {
        return Err(anyhow!(
            "Failed to get git path. Make sure you are in a git repository"
        ));
    }
    let path = String::from_utf8(output.stdout)?;
    Ok(PathBuf::from(path.trim()))
}

pub fn commit(commit_message: &str) -> Result<()> {
    let output = git_exec(&["commit", "-m", commit_message])?;
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

pub fn read_cached_commit() -> Result<String> {
    let commit_file_path = get_git_path()?.join("COMMIT_EDITMSG");
    let commit_message = fs::read_to_string(commit_file_path)?;
    Ok(commit_message)
}

pub fn write_cached_commit(commit_message: &str) -> Result<()> {
    fs::write(get_git_path()?.join("COMMIT_EDITMSG"), commit_message)?;
    Ok(())
}

pub fn pre_commit_check(pre_commit_command: Option<String>, message: &str) -> Result<()> {
    if let Some(command) = pre_commit_command {
        println!("Running pre-commit command...");
        let output = Command::new(command).env("MSG", message).output()?;
        let message = String::from_utf8_lossy(&output.stdout);
        println!("{message}");
        if !output.status.success() {
            let error_message = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!(
                "Pre-commit command failed with code: {}\n{}",
                output.status.code().unwrap_or_default(),
                error_message
            ));
        }
    }
    Ok(())
}
