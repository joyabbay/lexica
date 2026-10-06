use anyhow::Context;
use rusqlite::{OptionalExtension, params};

use crate::model::{Definition, Meaning, Word};

pub struct Database {
    conn: rusqlite::Connection,
}

const SCHEME: &str = include_str!("schema.sql");

impl Database {
    pub fn new(path: &str) -> anyhow::Result<Self> {
        let conn = rusqlite::Connection::open(path).context("Unable to connect to db")?;
        conn.execute_batch(SCHEME)
            .context("Unable to execute table schema")?;
        Ok(Self { conn })
    }
    pub fn insert_word(&self, word: Word) -> anyhow::Result<()> {
        let tx = self.conn.unchecked_transaction()?;

        // Insert the word itself.
        tx.execute(
            "INSERT INTO words (word, phonetic) VALUES (?1, ?2)",
            params![word.spelling, word.phonetic],
        )?;

        let word_id = tx.last_insert_rowid();

        // Insert meanings and definitions.
        for meaning in &word.meanings {
            tx.execute(
                "INSERT INTO meanings (word_id, part_of_speech)
                     VALUES (?1, ?2)",
                params![word_id, meaning.part_of_speech],
            )?;

            let meaning_id = tx.last_insert_rowid();

            for definition in &meaning.definitions {
                //TODO: Check this feature
                let synonym_string = serde_json::to_string(&definition.synonyms)?;
                tx.execute(
                    "INSERT INTO definitions (
                            meaning_id,
                            definition,
                            example,
                            synonyms
                        )
                        VALUES (?1, ?2, ?3,?4)",
                    params![meaning_id, definition.definition, definition.example,synonym_string],
                )?;
            }
        }

        tx.commit()?;

        Ok(())
    }
    pub fn get_word(
        &self,
        spelling: &str,
    ) -> anyhow::Result<Option<Word>> {
        // --------------------------------------------------
        // Get word
        // --------------------------------------------------

        let word = self.conn
            .query_row(
                "SELECT id, word, phonetic
             FROM words
             WHERE word = ?1",
                params![spelling],
                |row| {
                    Ok((
                        row.get::<_, i64>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, Option<String>>(2)?,
                    ))
                },
            )
            .optional()?;

        let Some((word_id, spelling, phonetic)) = word else {
            return Ok(None);
        };

        // --------------------------------------------------
        // Get meanings + definitions
        // --------------------------------------------------

        let mut stmt = self.conn.prepare(
            "SELECT
            m.id,
            m.part_of_speech,
            d.definition,
            d.example,
            d.synonyms
         FROM meanings m
         LEFT JOIN definitions d
            ON d.meaning_id = m.id
         WHERE m.word_id = ?1
         ORDER BY m.id, d.id",
        )?;

        let mut rows = stmt.query(params![word_id])?;

        let mut meanings: Vec<Meaning> = Vec::new();

        while let Some(row) = rows.next()? {
            let part_of_speech = row.get::<_, String>(1)?;

            // Find the Meaning corresponding to this row.
            let meaning_index = meanings
                .iter()
                .position(|meaning| {
                    meaning.part_of_speech.as_deref()
                        == Some(part_of_speech.as_str())
                });

            let meaning_index = match meaning_index {
                Some(index) => index,

                None => {
                    meanings.push(Meaning {
                        part_of_speech: Some(part_of_speech),
                        definitions: Vec::new(),
                    });

                    meanings.len() - 1
                }
            };

            // Because of LEFT JOIN, definition can be NULL.
            let Some(definition) = row.get::<_, Option<String>>(2)? else {
                continue;
            };

            let example = row.get::<_, Option<String>>(3)?;

            let synonyms_json = row
                .get::<_, Option<String>>(4)?
                .unwrap_or_else(|| "[]".to_string());

            let synonyms = serde_json::from_str::<Vec<String>>(&synonyms_json)
                .map_err(|_| rusqlite::Error::InvalidQuery)?;

            meanings[meaning_index]
                .definitions
                .push(Definition {
                    definition,
                    example,
                    synonyms,
                });
        }

        Ok(Some(Word {
            spelling,
            phonetic,
            meanings,
        }))
    }
}
