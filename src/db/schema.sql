--NOTE: Enforce foreign key constraints
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS words(
    id INTEGER PRIMARY KEY,
    word TEXT NOT NULL UNIQUE,
    phonetic TEXT,

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS meanings(
    id INTEGER PRIMARY KEY,
    word_id INTEGER NOT NULL,
    part_of_speech TEXT NOT NULL,

    FOREIGN KEY (word_id) REFERENCES words(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS definitions(
    id INTEGER PRIMARY KEY,
    meaning_id INTEGER NOT NULL,
    definition TEXT NOT NULL,
    example TEXT,
    synonyms TEXT,

    FOREIGN KEY (meaning_id) REFERENCES meanings(id) ON DELETE CASCADE
);


CREATE TABLE IF NOT EXISTS lists(
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS list_words(
    list_id INTEGER NOT NULL,
    word_id INTEGER NOT NULL,
    added_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (list_id, word_id),

    FOREIGN KEY (list_id) REFERENCES lists(id) ON DELETE CASCADE,
    FOREIGN KEY (word_id) REFERENCES words(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_list_words_word ON list_words(word_id);
CREATE INDEX IF NOT EXISTS idx_definitions_meaning ON definitions(meaning_id);
CREATE INDEX IF NOT EXISTS idx_meanings_word ON meanings(word_id);
