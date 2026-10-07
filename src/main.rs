use clap::{Parser, Subcommand};

use crate::cli::handle_command;
mod cli;
mod db;
mod dict;
mod model;

//TODO: Make path configurable or something
const PATH: &str = "app.db";

#[derive(Parser, Debug)]
#[command(
    name = "lexica",
    about = "simple word dictionary and bookmark app",
    version = "0.0.1"
)]
struct Cli {
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
    Invalidate{
        word: String,
    }
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
    let db = db::Database::new(PATH)?;
    let cli = Cli::parse();
    handle_command(cli, &db);
    Ok(())
}
