mod message_build;
mod prompt;

use anyhow::{anyhow, Result};
use message_build::MessageBuilder;
use prompt::Prompt;

use crate::commit_pattern::CommitPattern;

pub fn make_message_commit(pattern: CommitPattern) -> Result<String> {
    let mut message_inquirer = MessageInquirer::new(pattern.clone());
    let skip_commit = pattern.skip_commit;
    if !skip_commit.contains(&"commit_type".to_owned()) {
        message_inquirer.type_choice()?;
    }
    if !skip_commit.contains(&"commit_scope".to_owned()) {
        message_inquirer.scope_choice()?;
    }
    if !skip_commit.contains(&"commit_description".to_owned()) {
        message_inquirer.description()?;
    }
    if !skip_commit.contains(&"commit_body".to_owned()) {
        message_inquirer.body()?;
    }
    if !skip_commit.contains(&"commit_footer".to_owned()) {
        message_inquirer.footer()?;
    }
    message_inquirer.message()
}

struct MessageInquirer {
    commit_builder: MessageBuilder,
    prompt: Prompt,
    pattern: CommitPattern,
}

impl MessageInquirer {
    fn new(pattern: CommitPattern) -> Self {
        Self {
            commit_builder: MessageBuilder::new(pattern.config.clone()),
            prompt: Prompt::new(),
            pattern,
        }
    }

    fn type_choice(&mut self) -> Result<()> {
        let type_choice = self.prompt.select(
            &self.pattern.msg.commit_type,
            self.pattern.commit_types.clone(),
        )?;
        self.commit_builder.set_type(&type_choice);
        Ok(())
    }

    fn scope_choice(&mut self) -> Result<()> {
        let scope_choice = self.prompt.select(
            &self.pattern.msg.commit_scope,
            self.pattern.commit_scopes.clone(),
        )?;
        if scope_choice == "custom" {
            let custom_scope = self
                .prompt
                .required_input("Enter custom scope:", "Custom scope")?;
            self.commit_builder.set_scope(&custom_scope);
        } else if scope_choice != "none" {
            self.commit_builder.set_scope(&scope_choice);
        }
        Ok(())
    }

    fn description(&mut self) -> Result<()> {
        let description = self
            .prompt
            .required_input(&self.pattern.msg.commit_description, "Description")?;
        self.commit_builder.set_description(&description);
        Ok(())
    }

    fn body(&mut self) -> Result<()> {
        let body = self
            .prompt
            .optional_input(&self.pattern.msg.commit_body, "Commit body")?;
        if !body.is_empty() {
            self.commit_builder.set_body(&body);
        }
        Ok(())
    }

    fn footer(&mut self) -> Result<()> {
        let footer = self
            .prompt
            .optional_input(&self.pattern.msg.commit_footer, "Commit footer")?;
        if !footer.is_empty() {
            self.commit_builder.set_footer(&footer);
        }
        Ok(())
    }

    fn message(&mut self) -> Result<String> {
        println!(
            "\nThe commit message is:\n\n{}\n",
            self.commit_builder.message
        );
        let confirm = self.prompt.confirm("Do you want to apply the commit?")?;
        if !confirm {
            return Err(anyhow!("Operation was canceled by the user"));
        }
        Ok(self.commit_builder.message.clone())
    }
}
