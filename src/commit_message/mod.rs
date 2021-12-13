mod prompt;

use crate::config::{CommitPattern, Config};
use prompt::Prompt;

use anyhow::{anyhow, Result};

struct MessageBuilder {
    config: Config,
    message: String,
}

impl MessageBuilder {
    fn new(config: Config) -> MessageBuilder {
        MessageBuilder {
            config,
            message: String::new(),
        }
    }

    fn set_type(&mut self, commit_type: &str) {
        if let Some(prefix) = &self.config.type_prefix {
            self.message.push_str(prefix);
        }
        self.message.push_str(commit_type);
        if let Some(suffix) = &self.config.type_suffix {
            self.message.push_str(suffix);
        }
    }

    fn set_scope(&mut self, scope: &str) {
        self.message.push_str(&self.config.scope_prefix);
        self.message.push_str(scope);
        self.message.push_str(&self.config.scope_suffix);
    }

    fn set_description(&mut self, description: &str) {
        self.message.push_str(&self.config.subject_separator);
        self.message.push_str(format!(" {}", description).as_str());
    }

    fn set_body(&mut self, body: &str) {
        self.message.push_str(format!("\n\n{}", body).as_str());
    }

    fn set_footer(&mut self, footer: &str) {
        self.message.push_str(format!("\n\n{}", footer).as_str());
    }
}

pub fn make_message_commit(pattern: CommitPattern) -> Result<String> {
    let prompt = Prompt::new();
    let mut commit_builder = MessageBuilder::new(pattern.config);
    let type_choice = prompt.select(&pattern.msg.commit_type, pattern.commit_types)?;
    commit_builder.set_type(&type_choice);
    let scope_choice = prompt.select(&pattern.msg.commit_scope, pattern.commit_scopes)?;
    if scope_choice == "custom" {
        let custom_scope = prompt.required_input("Enter custom scope:", "Custom scope")?;
        commit_builder.set_scope(&custom_scope);
    } else if scope_choice != "none" {
        commit_builder.set_scope(&scope_choice);
    }
    let description = prompt.required_input(&pattern.msg.commit_description, "Description")?;
    commit_builder.set_description(&description);
    let body = prompt.optional_input(&pattern.msg.commit_body, "Commit body")?;
    if !body.is_empty() {
        commit_builder.set_body(&body);
    }
    let footer = prompt.optional_input(&pattern.msg.commit_footer, "Commit footer")?;
    if !footer.is_empty() {
        commit_builder.set_footer(&footer);
    }
    println!("\nThe commit message is:\n\n{}\n", commit_builder.message);
    let confirm = prompt.confirm("Do you want to apply the commit?")?;
    if !confirm {
        return Err(anyhow!("Operation was canceled by the user"));
    }
    Ok(commit_builder.message)
}
