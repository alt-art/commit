#[cfg(test)]
mod tests;

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use crate::DEFAULT_CONFIG_FILE;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub type_prefix: Option<String>,
    pub type_suffix: Option<String>,
    pub subject_separator: String,
    pub scope_prefix: String,
    pub scope_suffix: String,
}

impl Display for Type {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        write!(formatter, "{} - {}", self.name, self.description)
    }
}

#[derive(Clone, Deserialize)]
pub struct Type {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Clone)]
pub struct Messages {
    pub commit_type: String,
    pub commit_scope: String,
    pub commit_description: String,
    pub commit_body: String,
    pub commit_footer: String,
}

#[derive(Deserialize, Clone)]
pub struct CommitPattern {
    pub config: Config,
    pub commit_types: Vec<Type>,
    pub commit_scopes: Vec<Type>,
    #[serde(default)]
    pub skip_commit: Vec<String>,
    pub msg: Messages,
}

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
            .ok_or(anyhow!("Could not find config directory"))?
            .join("commit/commit.json");
        Ok(config_file)
    }
}

pub fn get_pattern(config_path: Option<PathBuf>) -> Result<CommitPattern> {
    let default_pattern_str = DEFAULT_CONFIG_FILE;
    let selected_config_path = select_custom_config_path(config_path)?;
    let pattern_str = get_config_path_content(&selected_config_path)
        .unwrap_or_else(|_| default_pattern_str.to_owned());
    serde_json::from_str(&pattern_str).context("Failed to parse commit pattern from file")
}
