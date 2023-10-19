use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub type_prefix: Option<String>,
    pub type_suffix: Option<String>,
    #[serde(default = "Config::subject_separator")]
    pub subject_separator: String,
    #[serde(default = "Config::scope_prefix")]
    pub scope_prefix: String,
    #[serde(default = "Config::scope_suffix")]
    pub scope_suffix: String,
    pub pre_commit: Option<String>,
}

impl Config {
    fn subject_separator() -> String {
        ": ".to_owned()
    }
    fn scope_prefix() -> String {
        "(".to_owned()
    }
    fn scope_suffix() -> String {
        ")".to_owned()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            type_prefix: None,
            type_suffix: None,
            subject_separator: Self::subject_separator(),
            scope_prefix: Self::scope_prefix(),
            scope_suffix: Self::scope_suffix(),
            pre_commit: None,
        }
    }
}
