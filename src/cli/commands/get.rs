use crate::{
    db::Database,
    dict::{DictionaryProvider, dictapi::DictApi},
};

pub fn invoke_get(spelling: &str, db: &Database) -> anyhow::Result<()> {
    if let Some(word) = db.get_word(spelling)? {
        println!("{word}");
    } else {
        println!("fetching from dictionary api...");
        let word = DictApi::get_word(spelling)?;
        println!("{word}");
        db.insert_word(word.clone())?;
    }
    Ok(())
}
