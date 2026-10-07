use serde::Deserialize;

use crate::{
    dict::DictionaryProvider,
    model::{Definition, Meaning, Word},
};

const API_URL: &str = "https://freedictionaryapi.com/api/v1/entries/en";

pub struct FreeDict;

#[derive(Debug, Deserialize)]
struct ApiResponse {
    word: String,
    entries: Vec<ApiEntry>,
}

#[derive(Debug, Deserialize)]
struct ApiEntry {
    #[serde(rename = "partOfSpeech")]
    part_of_speech: String,
    pronunciations: Vec<ApiPronunciation>,
    senses: Vec<ApiSense>,
}

#[derive(Debug, Deserialize)]
struct ApiPronunciation {
    text: String,
}

#[derive(Debug, Deserialize)]
struct ApiSense {
    definition: String,
    examples: Vec<String>,
    synonyms: Vec<String>,
}

impl From<ApiSense> for Definition {
    fn from(sense: ApiSense) -> Self {
        Self {
            definition: sense.definition,
            example: sense.examples.into_iter().next(),
            synonyms: sense.synonyms,
        }
    }
}

impl From<ApiEntry> for Meaning {
    fn from(entry: ApiEntry) -> Self {
        Self {
            part_of_speech: Some(entry.part_of_speech),
            definitions: entry
                .senses
                .into_iter()
                .map(Definition::from)
                .collect(),
        }
    }
}

impl From<ApiResponse> for Word {
    fn from(response: ApiResponse) -> Self {
        let phonetic = response
            .entries
            .iter()
            .flat_map(|entry| entry.pronunciations.iter())
            .map(|pronunciation| pronunciation.text.clone())
            .next();

        let meanings = response
            .entries
            .into_iter()
            .map(Meaning::from)
            .collect();

        Self {
            spelling: response.word,
            phonetic,
            meanings,
        }
    }
}

impl DictionaryProvider for FreeDict {
    fn get_word(word: &str) -> anyhow::Result<Word> {
        let client = reqwest::blocking::Client::new();

        let url = format!("{API_URL}/{word}");

        let response = client
            .get(url)
            .send()?
            .error_for_status()?;

        let response: ApiResponse = response.json()?;

        Ok(response.into())
    }
}
