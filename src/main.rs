use clap::Parser;
use cliflow::cli::arguments::{Cli, Commands};
use cliflow::error::Result;

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
    match cli.command {
        Commands::List => {
            let workflows = cliflow::infrastructure::embedded_loader::load_embedded_workflows()?;
            println!("{} workflows loaded", workflows.len());
            Ok(0)
        }
        Commands::Show { id } => {
            let workflows = cliflow::infrastructure::embedded_loader::load_embedded_workflows()?;
            let workflow = cliflow::application::show_workflow::show_workflow(&workflows, &id)?;
            cliflow::presentation::terminal_renderer::render_workflow(&workflow);
            Ok(0)
        }
        Commands::Search { query } => {
            println!("search is not implemented yet: {query}");
            Ok(0)
        }
    }
}
