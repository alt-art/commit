use anyhow::{anyhow, Result};

use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{exit, Command, Output, Stdio};
use std::{fs, thread};

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
        let mut process = Command::new(command)
            .env("MSG", message)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let stdout = process.stdout.take().expect("Unable to get stdout");
        let stderr = process.stderr.take().expect("Unable to get stderr");
        thread::spawn(move || {
            let lines = BufReader::new(stdout).lines();
            for line in lines {
                println!("{}", line.expect("Unable to get line"));
            }
        });
        thread::spawn(move || {
            let lines = BufReader::new(stderr).lines();
            for line in lines {
                eprintln!("{}", line.expect("Unable to get line"));
            }
        });
        let status = process.wait()?;
        if !status.success() {
            return Err(anyhow!("Pre-commit command failed with {}", status));
        }
    }
    Ok(())
}
