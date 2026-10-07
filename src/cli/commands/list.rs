pub fn invoke_create_list(list_name: &str,db: &crate::db::Database) -> anyhow::Result<()>{
    db.insert_list(list_name)
}

pub fn invoke_delete(list_name: &str, db: &crate::db::Database) -> anyhow::Result<()> {
    db.delete_list(list_name)
}

pub fn invoke_add_word_list(list_name: &str,word: &str,db: &crate::db::Database) -> anyhow::Result<()> {
    db.add_word_to_list(list_name, word)
}

pub fn invoke_remove_word_list(list_name: &str,word: &str,db: &crate::db::Database) -> anyhow::Result<()> {
    db.remove_word_from_list(list_name, word)
}

pub fn invoke_ls_list(db: &crate::db::Database) -> anyhow::Result<()>{
    let lists = db.show_lists()?;
    for list in lists {
        println!("{}", list);
    }
    Ok(())
}