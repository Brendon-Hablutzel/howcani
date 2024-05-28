use clap::{Parser, Subcommand};
use commands::{add_creds, ask, get_censored_creds, remove_creds};

pub mod commands;
pub mod creds;
pub mod models;

#[derive(Parser, Debug)]
#[command(name = "howcani", about = "A generative AI-powered CLI app for helping to use the command line", long_about = None)]
struct Cli {
    /// The subcommand to use
    #[command(subcommand)]
    cmd: Subcommands,
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    /// Get instructions for how to perform a task
    Ask {
        /// The task to get instructions for
        query: String,
    },
    /// Commands related to managing credentials
    #[command[subcommand]]
    Creds(CredsCommands),
}

#[derive(Subcommand, Debug)]
enum CredsCommands {
    /// Gets a partially-censored string of the credentials
    Get,
    /// Add model credentials
    Add,
    /// Remove model credentials
    Remove,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Subcommands::Ask { query } => ask(&query).await,
        Subcommands::Creds(creds_cmd) => match creds_cmd {
            CredsCommands::Get => get_censored_creds().await,
            CredsCommands::Add => add_creds().await,
            CredsCommands::Remove => remove_creds().await,
        },
    }
}
