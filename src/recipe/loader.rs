use super::Recipe;
use crate::error::{Error, Result};
use rust_embed::RustEmbed;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(RustEmbed)]
#[folder = "recipes/"]
struct EmbeddedRecipes;

pub fn load_recipes() -> Result<Vec<Recipe>> {
    let mut recipes = load_embedded_recipes()?;

    if let Some(path) = user_recipe_dir()
        && path.exists()
    {
        recipes.extend(load_filesystem_recipes(&path)?);
    }

    Ok(recipes)
}

pub fn load_embedded_recipes() -> Result<Vec<Recipe>> {
    let mut recipes = Vec::new();

    for file in EmbeddedRecipes::iter().filter(|path| path.ends_with(".toml")) {
        let content = EmbeddedRecipes::get(file.as_ref())
            .ok_or_else(|| Error::Message(format!("embedded recipe disappeared: {file}")))?;
        let text = std::str::from_utf8(content.data.as_ref())?;
        let recipe = toml::from_str::<Recipe>(text)?;
        recipes.push(recipe);
    }

    Ok(recipes)
}

fn load_filesystem_recipes(root: &Path) -> Result<Vec<Recipe>> {
    let mut recipes = Vec::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|entry| entry.ok())
    {
        let path = entry.path();
        if !path.is_file() || path.extension().and_then(|ext| ext.to_str()) != Some("toml") {
            continue;
        }

        let text = std::fs::read_to_string(path)?;
        let recipe = toml::from_str::<Recipe>(&text)?;
        recipes.push(recipe);
    }

    Ok(recipes)
}

fn user_recipe_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|path| path.join("cliflow").join("recipes"))
}
