use crate::config::Type;

use inquire::{
    required,
    ui::{Color, RenderConfig, Styled},
    Confirm, Select, Text,
};

use anyhow::Result;

pub struct Prompt {
    config: RenderConfig,
}

impl Prompt {
    pub fn new() -> Self {
        let default = RenderConfig::default();
        let prompt_prefix = Styled::new("-").with_fg(Color::LightGreen);
        let current_config = default.with_prompt_prefix(prompt_prefix);
        Self {
            config: current_config,
        }
    }

    pub fn optional_input(&self, prompt: &str, label: &str) -> Result<String> {
        let input = Text::new(prompt)
            .with_render_config(self.config)
            .with_help_message(format!("{}. Press Enter to skip", label).as_str())
            .prompt()?;
        Ok(input)
    }

    pub fn required_input(&self, prompt: &str, label: &str) -> Result<String> {
        let input = Text::new(prompt)
            .with_render_config(self.config)
            .with_validator(required!(format!("{} can't be empty", label).as_str()))
            .prompt()?;
        Ok(input)
    }

    pub fn select(&self, prompt: &str, choices: Vec<Type>) -> Result<String> {
        let choice = Select::new(prompt, choices)
            .with_render_config(self.config)
            .prompt()?;
        Ok(choice.name)
    }

    pub fn confirm(&self, prompt: &str) -> Result<bool> {
        let confirm = Confirm::new(prompt)
            .with_render_config(self.config)
            .with_default(true)
            .prompt()?;
        Ok(confirm)
    }
}
