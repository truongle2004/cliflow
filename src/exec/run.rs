use crate::display;
use crate::error::{Error, Result};
use crate::recipe::{Danger, Recipe};
use dialoguer::{Confirm, theme::ColorfulTheme};
use std::process::Command;

pub fn maybe_confirm(recipe: &Recipe, command: &str, yes: bool) -> Result<()> {
    if recipe.danger == Danger::Low {
        return Ok(());
    }

    display::print_resolved_command(command);

    if recipe.danger == Danger::Medium && yes {
        return Ok(());
    }

    let confirmed = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Run this {} danger command?", recipe.danger))
        .default(false)
        .interact()?;

    if !confirmed {
        return Err(Error::Message("aborted".to_string()));
    }

    Ok(())
}

pub fn run_command(command: &str) -> Result<i32> {
    let parts = shell_words::split(command)?;
    let Some((program, args)) = parts.split_first() else {
        return Err(Error::Message("recipe command is empty".to_string()));
    };

    let status = Command::new(program).args(args).status()?;

    Ok(status.code().unwrap_or(1))
}
