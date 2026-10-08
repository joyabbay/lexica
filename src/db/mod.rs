use std::path::Path;

use anyhow::Context;
use rusqlite::{OptionalExtension, params};

use crate::model::{Definition, Meaning, Word};

pub struct Database {
    conn: rusqlite::Connection,
}

const SCHEME: &str = include_str!("schema.sql");

impl Database {
    pub fn new(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();

        // Create the database file (and any missing parent directories, e.g.
        // when `LEXICA_DB` points into a directory that doesn't exist yet) if
        // it isn't there already.
        if !path.exists() {
            if let Some(parent) = path.parent()
                && !parent.as_os_str().is_empty()
            {
                std::fs::create_dir_all(parent).with_context(|| {
                    format!("Unable to create database directory {}", parent.display())
                })?;
            }

            std::fs::File::create(path)
                .with_context(|| format!("Unable to create database file {}", path.display()))?;

            println!("created new database at {}", path.display());
        }

        let conn = rusqlite::Connection::open(path).context("Unable to connect to db")?;
        conn.execute_batch(SCHEME)
            .context("Unable to execute table schema")?;
        Ok(Self { conn })
    }
    pub fn insert_list(&self, list_name: &str) -> anyhow::Result<()> {
        self.conn
            .execute("INSERT INTO lists (name) VALUES (?1)", params![list_name])?;
        Ok(())
    }
    pub fn delete_list(&self, list_name: &str) -> anyhow::Result<()> {
        self.conn
            .execute("DELETE FROM lists WHERE name = ?1", params![list_name])?;
        Ok(())
    }
    pub fn show_lists(&self) -> anyhow::Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT name FROM lists")?;
        let lists: Vec<String> = stmt
            .query_map([], |row| row.get::<_, String>(0))?
            .collect::<rusqlite::Result<Vec<String>>>()?;
        Ok(lists)
    }

    pub fn show_list(&self, list_name: &str) -> anyhow::Result<Vec<String>> {
        let list_id: Option<i64> = self
            .conn
            .query_row(
                "SELECT id FROM lists WHERE name = ?1",
                params![list_name],
                |row| row.get(0),
            )
            .optional()?;

        let Some(list_id) = list_id else {
            anyhow::bail!("list '{list_name}' not found");
        };

        let mut stmt = self.conn.prepare(
            "SELECT w.word
             FROM list_words lw
             JOIN words w ON w.id = lw.word_id
             WHERE lw.list_id = ?1
             ORDER BY lw.added_at, w.word",
        )?;

        let words: Vec<String> = stmt
            .query_map(params![list_id], |row| row.get::<_, String>(0))?
            .collect::<rusqlite::Result<Vec<String>>>()?;

        Ok(words)
    }

    pub fn invalidate_word(&self, word: &str) -> anyhow::Result<()> {
        self.conn
            .execute("DELETE FROM words WHERE word = ?1", params![word])?;
        Ok(())
    }
    pub fn remove_word_from_list(&self, list_name: &str, spelling: &str) -> anyhow::Result<()> {
        let tx = self.conn.unchecked_transaction()?;
        let word_id = tx.query_row(
            "SELECT id FROM words WHERE word=?1",
            params![spelling],
            |row| row.get::<_, i64>(0),
        )?;
        let list_id = tx.query_row(
            "SELECT id FROM lists WHERE name = ?1",
            params![list_name,],
            |row| row.get::<_, i64>(0),
        )?;
        tx.execute(
            "DELETE FROM list_words WHERE list_id = ?1 AND word_id = ?2",
            params![list_id, word_id],
        )?;
        tx.commit()?;
        Ok(())
    }
    pub fn add_word_to_list(&self, list_name: &str, word: &str) -> anyhow::Result<()> {
        let tx = self.conn.unchecked_transaction()?;
        let word_id = tx.query_row("SELECT id FROM words WHERE word=?1", params![word], |row| {
            row.get::<_, i64>(0)
        })?;
        let list_id = tx.query_row(
            "SELECT id FROM lists WHERE name = ?1",
            params![list_name,],
            |row| row.get::<_, i64>(0),
        )?;
        tx.execute(
            "INSERT INTO list_words (list_id,word_id) VALUES (?1,?2)",
            params![list_id, word_id],
        )?;
        tx.commit()?;
        Ok(())
    }
    pub fn insert_word(&self, word: Word) -> anyhow::Result<()> {
        let tx = self.conn.unchecked_transaction()?;

        // Insert the word itself. If it already exists (e.g. only differs
        // by casing) keep the cached entry and skip re-inserting meanings.
        let inserted = tx.execute(
            "INSERT INTO words (word, phonetic)
             VALUES (?1, ?2)
             ON CONFLICT(word) DO NOTHING",
            params![word.spelling, word.phonetic],
        )?;

        if inserted == 0 {
            tx.commit()?;
            return Ok(());
        }

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
                    params![
                        meaning_id,
                        definition.definition,
                        definition.example,
                        synonym_string
                    ],
                )?;
            }
        }

        tx.commit()?;

        Ok(())
    }
    pub fn search_word(&self, prefix: &str) -> anyhow::Result<Vec<String>> {
        let pattern = format!("{}%", prefix);
        let mut stmt = self
            .conn
            .prepare("SELECT word FROM words WHERE word LIKE ?1")?;
        let words: Vec<String> = stmt
            .query_map([pattern], |row| row.get::<_, String>(0))?
            .collect::<rusqlite::Result<Vec<String>>>()?;
        Ok(words)
    }
    pub fn get_word(&self, spelling: &str) -> anyhow::Result<Option<Word>> {
        let word = self
            .conn
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

        // Rows are ordered by `m.id`, so all rows belonging to the same
        // meaning are contiguous. Group by the meaning id rather than by
        // part of speech, which would incorrectly merge distinct meanings
        // that happen to share a part of speech.
        let mut current_meaning_id: Option<i64> = None;
        let mut current_meaning_index = 0usize;

        while let Some(row) = rows.next()? {
            let meaning_id = row.get::<_, i64>(0)?;

            let meaning_index = if current_meaning_id == Some(meaning_id) {
                current_meaning_index
            } else {
                let part_of_speech = row.get::<_, String>(1)?;

                meanings.push(Meaning {
                    part_of_speech,
                    definitions: Vec::new(),
                });

                current_meaning_id = Some(meaning_id);
                current_meaning_index = meanings.len() - 1;

                current_meaning_index
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
                .map_err(|e| anyhow::anyhow!("Failed to parse synonyms: {}", e))?;

            meanings[meaning_index].definitions.push(Definition {
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
