use serde::Deserialize;
use std::fmt::{Display, Formatter, Result};

#[derive(Deserialize, Clone)]
pub struct Config {
    pub type_prefix: Option<String>,
    pub type_suffix: Option<String>,
    pub subject_separator: String,
    pub scope_prefix: String,
    pub scope_suffix: String,
}

impl Display for Type {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{} - {}", self.name, self.description)
    }
}

#[derive(Clone, Deserialize)]
pub struct Type {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct Messages {
    pub commit_type: String,
    pub commit_scope: String,
    pub commit_description: String,
    pub commit_body: String,
    pub commit_footer: String,
}

#[derive(Deserialize)]
pub struct CommitPattern {
    pub config: Config,
    pub commit_types: Vec<Type>,
    pub commit_scopes: Vec<Type>,
    pub msg: Messages,
}

pub fn get_pattern() -> CommitPattern {
    let pattern_str = include_str!("../commit.json");
    let pattern: CommitPattern = match serde_json::from_str(pattern_str) {
        Ok(pattern) => pattern,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };
    pattern
}
