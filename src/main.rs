use clap::Parser;
use cliflow::cli::{Cli, Commands};
use cliflow::error::{Error, Result};
use cliflow::exec;
use cliflow::recipe::{Registry, load_recipes};

fn main() {
    let cli = Cli::parse();
    owo_colors::set_override(!cli.no_color);

    let code = match run(cli) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("error: {error:#}");
            1
        }
    };

    std::process::exit(code);
}

fn run(cli: Cli) -> Result<i32> {
    let registry = Registry::new(load_recipes()?);

    match cli.command {
        None | Some(Commands::Ui) => {
            cliflow::tui::run(&registry)?;
            Ok(0)
        }
        Some(Commands::DebugWorkflows) => {
            let workflows = cliflow::infrastructure::embedded_loader::load_embedded_workflows()?;
            println!("{}", workflows.len());
            Ok(0)
        }
        Some(Commands::Tools) => {
            cliflow::display::print_tools(&registry.namespaces());
            Ok(0)
        }
        Some(Commands::List { namespace }) => {
            let recipes = registry.list(namespace.as_deref());
            cliflow::display::print_recipe_list(&recipes);
            Ok(0)
        }
        Some(Commands::Search { query }) => {
            let results = cliflow::search::search(registry.all(), &query);
            cliflow::display::print_search_results(&results);
            Ok(0)
        }
        Some(Commands::Show { recipe }) => {
            let recipe = find_recipe(&registry, &recipe)?;
            cliflow::display::print_recipe(recipe);
            Ok(0)
        }
        Some(Commands::Run {
            recipe,
            dry_run,
            yes,
            set,
        }) => {
            let recipe = find_recipe(&registry, &recipe)?;
            let set_values = exec::resolve::parse_set_values(&set)?;
            let command = exec::resolve::resolve_command(recipe, &set_values)?;

            if dry_run {
                cliflow::display::print_resolved_command(&command);
                return Ok(0);
            }

            exec::run::maybe_confirm(recipe, &command, yes)?;
            exec::run::run_command(&command)
        }
    }
}

fn find_recipe<'a>(registry: &'a Registry, key: &str) -> Result<&'a cliflow::recipe::Recipe> {
    if !key.contains('/') {
        return Err(Error::Message(
            "recipe must be in namespace/id format".to_string(),
        ));
    }

    registry
        .get(key)
        .ok_or_else(|| Error::Message(format!("recipe not found: {key}")))
}
