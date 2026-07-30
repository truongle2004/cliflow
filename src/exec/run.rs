use crate::display;
use crate::error::Result;
use crate::recipe::{Danger, Recipe};
use anyhow::{Context, bail};
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
        .interact()
        .context("failed to read confirmation")?;

    if !confirmed {
        bail!("aborted");
    }

    Ok(())
}

pub fn run_command(command: &str) -> Result<i32> {
    let parts = shell_words::split(command).context("failed to parse command")?;
    let Some((program, args)) = parts.split_first() else {
        bail!("recipe command is empty");
    };

    let status = Command::new(program)
        .args(args)
        .status()
        .with_context(|| format!("failed to execute {program}"))?;

    Ok(status.code().unwrap_or(1))
}
