use clap::{ArgAction, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    version,
    about = "Curated command workflows for common developer tools"
)]
pub struct Cli {
    #[arg(short = 'v', global = true, action = ArgAction::Count)]
    pub verbose: u8,

    #[arg(long, global = true)]
    pub no_color: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// List workflows.
    List,
    /// Show one workflow.
    Show { id: String },
    /// Search workflows.
    Search { query: String },
}
