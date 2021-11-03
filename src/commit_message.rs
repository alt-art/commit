use inquire::{
    error::InquireError,
    ui::{Color, RenderConfig, Styled},
    Confirm, Select, Text,
};

use crate::config::{CommitPattern, Config};

struct MessageBuilder {
    config: Config,
    commit_type: String,
    commit_scope: Option<String>,
    commit_description: String,
    commit_body: Option<String>,
    commit_footer: Option<String>,
}

impl MessageBuilder {
    fn new(config: Config) -> MessageBuilder {
        MessageBuilder {
            config,
            commit_type: String::new(),
            commit_scope: None,
            commit_description: String::new(),
            commit_body: None,
            commit_footer: None,
        }
    }
    fn build(&self) -> String {
        let mut message = String::new();
        if let Some(prefix) = &self.config.type_prefix {
            message.push_str(prefix);
        }
        message.push_str(&self.commit_type);
        if let Some(suffix) = &self.config.type_suffix {
            message.push_str(suffix);
        }
        if let Some(scope) = &self.commit_scope {
            message.push_str(&self.config.scope_prefix);
            message.push_str(scope);
            message.push_str(&self.config.scope_suffix);
        }
        message.push_str(&self.config.subject_separator);
        message.push_str(&format!(" {}", &self.commit_description));
        if let Some(body) = &self.commit_body {
            message.push_str("\n\n");
            message.push_str(body);
        }
        if let Some(footer) = &self.commit_footer {
            message.push_str("\n\n");
            message.push_str(footer);
        }
        message
    }
}

pub fn make_message_commit(pattern: &CommitPattern) -> Result<String, InquireError> {
    let default = RenderConfig::default();
    let prompt_prefix = Styled::new("-").with_fg(Color::LightGreen);
    let current_config = default.with_prompt_prefix(prompt_prefix);
    let mut commit_builder = MessageBuilder::new(pattern.config.clone());
    let type_choice = Select::new("What type of commit you will made?", pattern.types.clone())
        .with_render_config(current_config)
        .prompt()?;
    commit_builder.commit_type = type_choice.name;
    let scope_choice = Select::new(
        "What scope of commit you will made? (Optional)",
        pattern.scopes.clone(),
    )
    .with_render_config(current_config)
    .prompt()?;
    if scope_choice.name == "custom" {
        let custom_scope = Text::new("Enter custom scope:")
            .with_render_config(current_config)
            .prompt()?;
        commit_builder.commit_scope = Some(custom_scope);
    } else if scope_choice.name != "none" {
        commit_builder.commit_scope = Some(scope_choice.name);
    } else {
        commit_builder.commit_scope = None;
    }
    let description = Text::new("Write a SHORT, IMPERATIVE tense description of the change:")
        .with_render_config(current_config)
        .prompt()?;
    if description.is_empty() {
        println!("The description can't be empty");
        std::process::exit(1);
    }
    commit_builder.commit_description = description;
    let body = Text::new("Provide a LONGER description of the change (optional):")
        .with_render_config(current_config)
        .with_help_message("Commit body. Press Enter to skip")
        .prompt()?;
    if !body.is_empty() {
        commit_builder.commit_body = Some(body);
    }
    let footer = Text::new("List any ISSUES CLOSED by this change (optional). E.g.: #31, #34:")
        .with_render_config(current_config)
        .with_help_message("Commit footer. Press Enter to skip")
        .prompt()?;
    if !footer.is_empty() {
        commit_builder.commit_footer = Some(footer);
    }
    let message = commit_builder.build();
    println!("\n  The commit message is:\n\n  {}\n", message);
    let confirm = Confirm::new("Do you want to apply the commit?")
        .with_render_config(current_config)
        .with_default(true)
        .prompt()?;
    if !confirm {
        return Err(InquireError::OperationCanceled);
    }
    Ok(message)
}
