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

fn search_config_file_on_parents() -> Result<Option<PathBuf>> {
    let current_dir = std::env::current_dir()?;
    let mut current_dir = current_dir.as_path();
    loop {
        let config_file = current_dir.join("commit.json");
        if config_file.is_file() {
            return Ok(Some(config_file));
        }
        if let Some(parent) = current_dir.parent() {
            current_dir = parent;
        } else {
            break;
        }
    }
    Ok(None)
}

fn get_config_path() -> Result<PathBuf> {
    Ok(search_config_file_on_parents()?.unwrap_or_else(|| {
        dirs::config_dir()
            .expect("Could not find config directory")
            .join("commit/commit.json")
    }))
}

pub fn get_pattern(config_path: Option<PathBuf>) -> Result<CommitPattern> {
    let selected_config_path = select_custom_config_path(config_path)?;
    let pattern_str =
        get_config_path_content(selected_config_path).unwrap_or_else(|_| "{}".to_owned());
    serde_json::from_str(&pattern_str).context("Failed to parse commit pattern from file")
}
