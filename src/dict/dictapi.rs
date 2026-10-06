//This uses free API provider `dictionaryapi.dev`
use serde::Deserialize;

//TODO: Again make this configurable
// API endpoint
const API_KEY: &str = "https://api.dictionaryapi.dev/api/v2/entries/en/";

pub struct DictApi;

/*
*
[
   {
     "word": "hello",
     "phonetic": "həˈləʊ",
     "phonetics": [
       {
         "text": "həˈləʊ",
         "audio": "//ssl.gstatic.com/dictionary/static/sounds/20200429/hello--_gb_1.mp3"
       },
       {
         "text": "hɛˈləʊ"
       }
     ],
     "origin": "early 19th century: variant of earlier hollo ; related to holla.",
     "meanings": [
       {
         "partOfSpeech": "exclamation",
         "definitions": [
           {
             "definition": "used as a greeting or to begin a phone conversation.",
             "example": "hello there, Katie!",
             "synonyms": [],
             "antonyms": []
           }
         ]
       },
       {
         "partOfSpeech": "noun",
         "definitions": [
           {
             "definition": "an utterance of ‘hello’; a greeting.",
             "example": "she was getting polite nods and hellos from people",
             "synonyms": [],
             "antonyms": []
           }
         ]
       },
       {
         "partOfSpeech": "verb",
         "definitions": [
           {
             "definition": "say or shout ‘hello’.",
             "example": "I pressed the phone button and helloed",
             "synonyms": [],
             "antonyms": []
           }
         ]
       }
     ]
   }
 ]

*
*/

use crate::{
    dict::DictionaryProvider,
    model::{Definition, Meaning, Word},
};

#[derive(Debug, Deserialize)]
struct ApiEntry {
    word: String,
    phonetic: Option<String>,
    meanings: Vec<ApiMeaning>,
}

#[derive(Debug, Deserialize)]
struct ApiMeaning {
    #[serde(rename = "partOfSpeech")]
    part_of_speech: Option<String>,
    definitions: Vec<ApiDefinition>,
}

#[derive(Debug, Deserialize)]
struct ApiDefinition {
    definition: String,
    example: Option<String>,
    synonyms: Vec<String>,
}


impl From<ApiDefinition> for Definition {
    fn from(definition: ApiDefinition) -> Self {
        Self {
            definition: definition.definition,
            example: definition.example,
            synonyms: definition.synonyms,
        }
    }
}
impl From<ApiMeaning> for Meaning {
    fn from(meaning: ApiMeaning) -> Self {
        Self {
            part_of_speech: meaning.part_of_speech,
            definitions: meaning
                .definitions
                .into_iter()
                .map(Definition::from)
                .collect(),
        }
    }
}
impl From<Vec<ApiEntry>> for Word {
    fn from(entries: Vec<ApiEntry>) -> Self {
        let mut entries = entries.into_iter();

        let first = entries
            .next()
            .expect("cannot create Word from empty entries");

        let mut word = Word {
            spelling: first.word,
            phonetic: first.phonetic,
            meanings: Vec::new(),
        };

        add_meanings(&mut word.meanings, first.meanings);

        for entry in entries {
            if word.phonetic.is_none() {
                word.phonetic = entry.phonetic;
            }

            add_meanings(&mut word.meanings, entry.meanings);
        }

        word
    }
}

fn add_meanings(meanings: &mut Vec<Meaning>, api_meanings: Vec<ApiMeaning>) {
    for api_meaning in api_meanings {
        let new_meaning = Meaning::from(api_meaning);

        if let Some(existing) = meanings
            .iter_mut()
            .find(|meaning| meaning.part_of_speech == new_meaning.part_of_speech)
        {
            existing.definitions.extend(new_meaning.definitions);
        } else {
            meanings.push(new_meaning);
        }
    }
}

impl DictionaryProvider for DictApi {
    fn get_word(word: &str) -> anyhow::Result<Word> {
        let client = reqwest::blocking::Client::new();
        let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);

        let response = client.get(url).send()?.error_for_status()?;

        let entries: Vec<ApiEntry> = response.json()?;

        if entries.is_empty() {
            return Err(anyhow::anyhow!("no dictionary entry found"));
        }

        Ok(entries.into())
        // pollster::block_on(Self::lookup(word))
    }
}
// impl DictApi {
//     pub fn lookup(spelling: &str) -> anyhow::Result<Word> {
//     }
// }

mod tests {
    use super::{DictApi, DictionaryProvider};

    #[test]
    fn test_get_word() -> anyhow::Result<()> {
        let word = DictApi::get_word("hello")?;
        println!("{word}");
        Ok(())
    }
}
