use crate::{db::Database, dict::DictionaryProvider};

pub fn invoke_get(
    spelling: &str,
    db: &Database,
    provider: &dyn DictionaryProvider,
) -> anyhow::Result<()> {
    if let Some(word) = db.get_word(spelling)? {
        println!("{word}");
    } else {
        println!("fetching from dictionary api...");
        let word = provider.get_word(spelling)?;
        println!("{word}");
        db.insert_word(word)?;
    }
    Ok(())
}

pub fn invoke_invalidate(word: &str, db: &Database) -> anyhow::Result<()> {
    db.invalidate_word(word)
}

pub fn invoke_search(word: &str, db: &crate::db::Database) -> anyhow::Result<()> {
    let words = db.search_word(word)?;
    for word in words {
        println!("{}", word);
    }

    Ok(())
}
