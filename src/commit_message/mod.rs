mod message_build;
mod prompt;

use anyhow::{anyhow, Result};

use crate::config::CommitPattern;
use message_build::MessageBuilder;
use prompt::Prompt;

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
