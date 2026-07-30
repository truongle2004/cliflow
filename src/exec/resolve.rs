use crate::error::{Error, Result};
use crate::recipe::Recipe;
use dialoguer::{Input, theme::ColorfulTheme};
use std::collections::BTreeMap;

pub fn parse_set_values(values: &[String]) -> Result<BTreeMap<String, String>> {
    let mut parsed = BTreeMap::new();

    for value in values {
        let Some((name, val)) = value.split_once('=') else {
            return Err(Error::Message(format!(
                "--set must use name=value format: {value}"
            )));
        };
        if name.is_empty() {
            return Err(Error::Message(format!(
                "--set name cannot be empty: {value}"
            )));
        }
        parsed.insert(name.to_string(), val.to_string());
    }

    Ok(parsed)
}

pub fn resolve_command(recipe: &Recipe, set_values: &BTreeMap<String, String>) -> Result<String> {
    let mut resolved = recipe.command.clone();

    for arg in &recipe.args {
        let value = match set_values.get(&arg.name) {
            Some(value) => value.clone(),
            None => prompt_for_arg(arg)?,
        };

        resolved = resolved.replace(&format!("{{{}}}", arg.name), &value);
    }

    Ok(resolved)
}

fn prompt_for_arg(arg: &crate::recipe::Arg) -> Result<String> {
    let theme = ColorfulTheme::default();
    let mut input = Input::<String>::with_theme(&theme).with_prompt(&arg.prompt);

    if let Some(default) = &arg.default {
        input = input.default(default.clone());
    }

    Ok(input.interact_text()?)
}
