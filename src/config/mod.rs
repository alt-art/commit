#[cfg(test)]
mod tests;

use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use crate::commit_pattern::CommitPattern;

fn get_config_path_content(config_path: impl AsRef<Path>) -> Result<String> {
    let content = fs::read_to_string(config_path)?;
    Ok(content)
}

fn select_custom_config_path(config: Option<PathBuf>) -> Result<PathBuf> {
    match config {
        Some(config_path) => {
            if config_path.exists() {
                if !config_path.is_file() {
                    return Err(anyhow!(
                        "Config file path is not a file: {}",
                        config_path.display()
                    ));
                }
                Ok(config_path)
            } else {
                Err(anyhow!(
                    "Config file does not exist: {}",
                    config_path.display()
                ))
            }
        }
        None => get_config_path(),
    }
}

fn get_config_path() -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;
    let current_file = current_dir.join("commit.json");
    if current_file.is_file() {
        Ok(current_file)
    } else {
        let config_file = dirs::config_dir()
            .ok_or_else(|| anyhow!("Could not find config directory"))?
            .join("commit/commit.json");
        Ok(config_file)
    }
}

pub fn get_pattern(config_path: Option<PathBuf>) -> Result<CommitPattern> {
    let selected_config_path = select_custom_config_path(config_path)?;
    let pattern_str =
        get_config_path_content(selected_config_path).unwrap_or_else(|_| "{}".to_owned());
    serde_json::from_str(&pattern_str).context("Failed to parse commit pattern from file")
}
