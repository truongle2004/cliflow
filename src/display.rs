use crate::recipe::{Danger, Recipe};
use crate::search::SearchResult;
use owo_colors::OwoColorize;

pub fn print_tools(namespaces: &[&str]) {
    for namespace in namespaces {
        println!("{namespace}");
    }
}

pub fn print_recipe_list(recipes: &[&Recipe]) {
    for recipe in recipes {
        println!("{}  {}", recipe.key().cyan(), recipe.title);
    }
}

pub fn print_search_results(results: &[SearchResult<'_>]) {
    for result in results {
        println!("{}  {}", result.recipe.key().cyan(), result.recipe.title);
    }
}

pub fn print_recipe(recipe: &Recipe) {
    println!("{}", recipe.title.bold());
    println!("{}", recipe.key().cyan());

    if !recipe.description.is_empty() {
        println!("\n{}", recipe.description);
    }

    println!("\n{}", "Command".bold());
    println!("  {}", recipe.command.green());

    if !recipe.example.is_empty() {
        println!("\n{}", "CLI Example".bold());
        println!("  {}", recipe.example);
    }

    if !recipe.args.is_empty() {
        println!("\n{}", "Placeholders".bold());
        for arg in &recipe.args {
            match &arg.default {
                Some(default) => println!("  {}: {} [{}]", arg.name.yellow(), arg.prompt, default),
                None => println!("  {}: {}", arg.name.yellow(), arg.prompt),
            }
        }
    }

    if !recipe.tags.is_empty() {
        println!("\n{}", "Tags".bold());
        println!("  {}", recipe.tags.join(", "));
    }

    println!("\n{}", "Danger".bold());
    println!("  {}", danger_label(recipe.danger));
}

pub fn print_resolved_command(command: &str) {
    println!("{}", "Resolved command".bold());
    println!("  {}", command.green());
}

fn danger_label(danger: Danger) -> String {
    match danger {
        Danger::Low => danger.to_string().green().to_string(),
        Danger::Medium => danger.to_string().yellow().to_string(),
        Danger::High => danger.to_string().red().bold().to_string(),
    }
}
