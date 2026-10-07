use crate::model::Word;
pub mod dictapi;
pub mod freedict;

pub trait DictionaryProvider {
    fn get_word(word: &str) -> anyhow::Result<Word>;
}
