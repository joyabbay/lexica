use std::path::PathBuf;

use crate::Cli;
use crate::dict::Provider;

/// Database path used when neither the CLI nor the environment specifies one.
const DEFAULT_DATABASE: &str = "app.db";

/// Fully resolved settings used by the application.
///
/// Each value is taken from the matching CLI flag, falling back to the
/// environment (`LEXICA_PROVIDER` / `LEXICA_DB`), then a built-in default.
#[derive(Debug)]
pub struct Settings {
    pub provider: Provider,
    pub database: PathBuf,
}

impl Settings {
    pub fn resolve(cli: &Cli) -> anyhow::Result<Self> {
        let provider = match cli
            .provider
            .clone()
            .or_else(|| std::env::var("LEXICA_PROVIDER").ok())
        {
            Some(name) => Provider::from_name(&name)?,
            None => Provider::default(),
        };

        let database = cli
            .database
            .clone()
            .or_else(|| std::env::var("LEXICA_DB").ok())
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(DEFAULT_DATABASE));

        Ok(Self { provider, database })
    }
}
