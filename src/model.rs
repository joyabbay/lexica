use std::fmt;

#[derive(Debug, Clone)]
pub struct Word {
    pub spelling: String,
    pub phonetic: Option<String>,
    pub meanings: Vec<Meaning>,
}

#[derive(Debug, Clone)]
pub struct Meaning {
    pub part_of_speech: String,
    pub definitions: Vec<Definition>,
}

#[derive(Debug, Clone)]
pub struct Definition {
    pub definition: String,
    pub example: Option<String>,
    pub synonyms: Vec<String>,
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.spelling)?;

        if let Some(phonetic) = &self.phonetic {
            writeln!(f, "  /{}/", phonetic)?;
        }

        for meaning in &self.meanings {
            writeln!(f, "{meaning}")?;
        }

        Ok(())
    }
}

impl fmt::Display for Meaning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  {}", self.part_of_speech)?;

        for (i, definition) in self.definitions.iter().enumerate() {
            writeln!(f, "  {}. {}", i + 1, definition)?;
        }

        Ok(())
    }
}

impl fmt::Display for Definition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.definition)?;

        if let Some(example) = &self.example {
            write!(f, "\n     Example: \"{example}\"")?;
        }

        if !self.synonyms.is_empty() {
            write!(f, "\n     Synonyms: {}", self.synonyms.join(", "))?;
        }

        Ok(())
    }
}
