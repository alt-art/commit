use anyhow::{anyhow, Result};

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{exit, Command};

pub fn commit_as_hook(commit_message: &str) -> Result<()> {
    let output = Command::new("git")
        .args(&["rev-parse", "--absolute-git-dir"])
        .output();
    match output {
        Ok(output) => {
            if !output.status.success() {
                return Err(anyhow!("Could not get git directory"));
            }
            let git_dir = PathBuf::from(String::from_utf8_lossy(&output.stdout).trim());
            let commit_file_path = git_dir.join("COMMIT_EDITMSG");
            fs::write(commit_file_path, commit_message)?;
        }
        Err(e) => {
            return Err(anyhow!(
                "Failed to run git. Make sure git is installed\nAdditional info: {}",
                e
            ));
        }
    }
    Ok(())
}

pub fn commit(commit_message: &str) -> Result<()> {
    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .output();
    match output {
        Ok(output) => {
            std::io::stdout().write_all(&output.stdout)?;
            std::io::stderr().write_all(&output.stderr)?;
            exit(
                output
                    .status
                    .code()
                    .ok_or_else(|| anyhow!("Could not get exit code"))?,
            );
        }
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Failed to run git. Make sure git is installed\nAdditional info: {}",
                e
            ));
        }
    };
}
