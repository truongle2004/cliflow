use clap::{ArgAction, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about = "Curated command recipes for common developer tools")]
pub struct Cli {
    #[arg(short = 'v', global = true, action = ArgAction::Count)]
    pub verbose: u8,

    #[arg(long, global = true)]
    pub no_color: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Open the interactive search UI.
    Ui,
    /// Print loaded workflow count.
    #[command(hide = true)]
    DebugWorkflows,
    /// List all namespaces.
    Tools,
    /// List recipes, optionally filtered to one namespace.
    List { namespace: Option<String> },
    /// Search across recipe id, title, description, and tags.
    Search { query: String },
    /// Show a recipe without running it.
    Show { recipe: String },
    /// Resolve placeholders and run a recipe.
    Run {
        recipe: String,
        /// Show the resolved command without executing it.
        #[arg(long)]
        dry_run: bool,
        /// Skip confirmation for medium-danger recipes. High-danger recipes always confirm.
        #[arg(long)]
        yes: bool,
        /// Set a placeholder value as name=value. Repeat for multiple values.
        #[arg(long = "set", value_name = "NAME=VALUE")]
        set: Vec<String>,
    },
}
