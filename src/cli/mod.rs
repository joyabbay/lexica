use crate::ListOp;
use crate::{
    Cli, Command,
    db::Database,
    dict::{DictionaryProvider, Provider},
};
mod commands;

pub fn handle_command(
    cli: Cli,
    db: &Database,
    provider: &dyn DictionaryProvider,
) -> anyhow::Result<()> {
    match cli.command {
        Command::Get { word } => commands::invoke_get(&word, db, provider),
        Command::Search { prefix } => commands::invoke_search(&prefix, db),
        Command::Invalidate { word } => commands::invoke_invalidate(&word, db),
        Command::Providers => {
            for name in Provider::ALL {
                println!("{name}");
            }
            Ok(())
        }
        Command::List { operation } => {
            match operation {
                //TODO: Read sql join again
                ListOp::Create { name } => commands::invoke_create_list(&name, db),
                ListOp::Add { list, word } => commands::invoke_add_word_list(&list, &word, db),
                ListOp::Delete { name } => commands::invoke_delete(&name, db),
                ListOp::Ls => commands::invoke_ls_list(db),
                ListOp::Remove { list, word } => {
                    commands::invoke_remove_word_list(&list, &word, db)
                }
                ListOp::Show { list } => commands::invoke_show_list(&list, db),
            }
        }
    }
}
