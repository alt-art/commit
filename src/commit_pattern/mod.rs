mod config;
mod messages;

use serde::Deserialize;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub use config::Config;
use messages::Messages;

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
pub struct CommitPattern {
    #[serde(default)]
    pub config: Config,
    #[serde(default = "CommitPattern::commit_types")]
    pub commit_types: Vec<Type>,
    #[serde(default = "CommitPattern::commit_scopes")]
    pub commit_scopes: Vec<Type>,
    #[serde(default)]
    pub skip_commit: Vec<String>,
    #[serde(default)]
    pub msg: Messages,
}

impl CommitPattern {
    fn commit_types() -> Vec<Type> {
        vec![
            Type {
                name: "feat".to_owned(),
                description: "A new feature".to_owned(),
            },
            Type {
                name: "fix".to_owned(),
                description: "A bug fix".to_owned(),
            },
            Type {
                name: "docs".to_owned(),
                description: "Documentation only changes".to_owned(),
            },
            Type {
                name: "style".to_owned(),
                description: "Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)".to_owned(),
            },
            Type {
                name: "refactor".to_owned(),
                description: "A code change that neither fixes a bug nor adds a feature".to_owned(),
            },
            Type {
                name: "perf".to_owned(),
                description: "A code change that improves performance".to_owned(),
            },
            Type {
                name: "test".to_owned(),
                description: "Adding missing tests or correcting existing tests".to_owned(),
            },
            Type {
                name: "chore".to_owned(),
                description: "Other changes that don't modify src or test files".to_owned(),
            },
        ]
    }
    fn commit_scopes() -> Vec<Type> {
        vec![
            Type {
                name: "custom".to_owned(),
                description: "Custom scope".to_owned(),
            },
            Type {
                name: "none".to_owned(),
                description: "No scope".to_owned(),
            },
        ]
    }
}

impl Default for CommitPattern {
    fn default() -> Self {
        Self {
            config: Config::default(),
            commit_types: Self::commit_types(),
            commit_scopes: Self::commit_scopes(),
            skip_commit: vec![],
            msg: Messages::default(),
        }
    }
}
