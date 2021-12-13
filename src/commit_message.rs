use crate::config::{CommitPattern, Config};

use anyhow::{anyhow, Result};
use inquire::{
    required,
    ui::{Color, RenderConfig, Styled},
    Confirm, Select, Text,
};

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
    let default = RenderConfig::default();
    let prompt_prefix = Styled::new("-").with_fg(Color::LightGreen);
    let current_config = default.with_prompt_prefix(prompt_prefix);
    let mut commit_builder = MessageBuilder::new(pattern.config.clone());
    let type_choice = Select::new(&pattern.msg.commit_type, pattern.commit_types.clone())
        .with_render_config(current_config)
        .prompt()?;
    commit_builder.set_type(&type_choice.name);
    let scope_choice = Select::new(&pattern.msg.commit_scope, pattern.commit_scopes.clone())
        .with_render_config(current_config)
        .prompt()?;
    if scope_choice.name == "custom" {
        let custom_scope = Text::new("Enter custom scope:")
            .with_render_config(current_config)
            .with_validator(required!("Custom scope can't be empty"))
            .prompt()?;
        commit_builder.set_scope(&custom_scope);
    } else if scope_choice.name != "none" {
        commit_builder.set_scope(&scope_choice.name);
    }
    let description = Text::new(&pattern.msg.commit_description)
        .with_render_config(current_config)
        .with_validator(required!("The description can't be empty"))
        .prompt()?;
    commit_builder.set_description(&description);
    let body = Text::new(&pattern.msg.commit_body)
        .with_render_config(current_config)
        .with_help_message("Commit body. Press Enter to skip")
        .prompt()?;
    if !body.is_empty() {
        commit_builder.set_body(&body);
    }
    let footer = Text::new(&pattern.msg.commit_footer)
        .with_render_config(current_config)
        .with_help_message("Commit footer. Press Enter to skip")
        .prompt()?;
    if !footer.is_empty() {
        commit_builder.set_footer(&footer);
    }
    println!("\nThe commit message is:\n\n{}\n", commit_builder.message);
    let confirm = Confirm::new("Do you want to apply the commit?")
        .with_render_config(current_config)
        .with_default(true)
        .prompt()?;
    if !confirm {
        return Err(anyhow!("Operation was canceled by the user"));
    }
    Ok(commit_builder.message)
}
