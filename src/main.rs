use clap::{Parser, Subcommand};

use crate::cli::handle_command;
mod cli;
mod config;
mod db;
mod dict;
mod model;

#[derive(Parser, Debug)]
#[command(
    name = "lexica",
    about = "simple word dictionary and bookmark app",
    version = "0.0.1"
)]
struct Cli {
    /// Dictionary provider to fetch words from (overrides `LEXICA_PROVIDER`).
    /// Run `lexica providers` to list the options.
    #[arg(long, global = true, value_name = "NAME")]
    provider: Option<String>,

    /// Path to the SQLite database (overrides `LEXICA_DB`).
    #[arg(long = "db", global = true, value_name = "PATH")]
    database: Option<String>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Get {
        word: String,
    },
    Search {
        prefix: String,
    },
    List {
        #[command(subcommand)]
        operation: ListOp,
    },
    Invalidate {
        word: String,
    },
    /// List the dictionary providers selectable with `--provider`.
    Providers,
}

#[derive(Subcommand, Debug)]
enum ListOp {
    Create { name: String },
    Delete { name: String },
    //shows all lists
    Ls,
    Show { list: String },
    Add { list: String, word: String },
    Remove { list: String, word: String },
}
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let settings = config::Settings::resolve(&cli)?;
    let db = db::Database::new(&settings.database)?;
    handle_command(cli, &db, &settings.provider)?;
    Ok(())
}
