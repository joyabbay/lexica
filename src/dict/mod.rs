use anyhow::bail;

use crate::model::Word;

pub mod dictapi;
pub mod freedict;

/// A source that can look up a word's definition.
pub trait DictionaryProvider {
    fn get_word(&self, word: &str) -> anyhow::Result<Word>;
}

/// The set of dictionary providers `lexica` can fetch words from.
///
/// To register a new provider, add a variant here, an arm in
/// [`Provider::from_name`], a name in [`Provider::ALL`] and an arm in the
/// [`DictionaryProvider`] impl below.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Provider {
    #[default]
    FreeDict,
    DictApi,
}

impl Provider {
    /// Provider names accepted by [`Provider::from_name`], used in help and
    /// error messages.
    pub const ALL: &'static [&'static str] = &["freedict", "dictapi"];

    /// Parses a provider name (case-insensitive, with a few aliases). Unknown
    /// names produce an error listing the available providers.
    pub fn from_name(name: &str) -> anyhow::Result<Self> {
        match name.trim().to_ascii_lowercase().as_str() {
            "freedict" | "free" => Ok(Provider::FreeDict),
            "dictapi" | "dictionaryapi" => Ok(Provider::DictApi),
            other => bail!(
                "unknown dictionary provider '{other}' (available: {})",
                Provider::ALL.join(", ")
            ),
        }
    }
}

impl DictionaryProvider for Provider {
    fn get_word(&self, word: &str) -> anyhow::Result<Word> {
        match self {
            Provider::FreeDict => freedict::FreeDict.get_word(word),
            Provider::DictApi => dictapi::DictApi.get_word(word),
        }
    }
}
