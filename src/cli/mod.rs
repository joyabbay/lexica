use crate::{Cli, Command, db::Database};
mod commands;
pub fn handle_command(cli: Cli, db: &Database) -> anyhow::Result<()> {
    match cli.command {
        Command::Get { word } => {
            commands::invoke_get(&word, db)
        }
        Command::Search { prefix } => {
            todo!()
        }
        Command::List { operation } => {
            todo!()
        }
    }
}
