#[cfg(test)]
mod tests;

use crate::config::Config;

pub struct MessageBuilder {
    config: Config,
    pub message: String,
}

impl MessageBuilder {
    pub fn new(config: Config) -> MessageBuilder {
        MessageBuilder {
            config,
            message: String::new(),
        }
    }

    pub fn set_type(&mut self, commit_type: &str) {
        if let Some(prefix) = &self.config.type_prefix {
            self.message.push_str(prefix);
        }
        self.message.push_str(commit_type);
        if let Some(suffix) = &self.config.type_suffix {
            self.message.push_str(suffix);
        }
    }

    pub fn set_scope(&mut self, scope: &str) {
        self.message.push_str(&self.config.scope_prefix);
        self.message.push_str(scope);
        self.message.push_str(&self.config.scope_suffix);
    }

    pub fn set_description(&mut self, description: &str) {
        self.message.push_str(&self.config.subject_separator);
        self.message.push_str(description);
    }

    pub fn set_body(&mut self, body: &str) {
        self.message.push_str(format!("\n\n{}", body).as_str());
    }

    pub fn set_footer(&mut self, footer: &str) {
        self.message.push_str(format!("\n\n{}", footer).as_str());
    }
}
