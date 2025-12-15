use std::env;

use anyhow::Context;

const DATABASE_URL_KEY: &str = "DATABASE_URL";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Config> {
        let database_url = load_env(DATABASE_URL_KEY)?;

        Ok(Config { database_url })
    }
}

fn load_env(key: &str) -> anyhow::Result<String> {
    env::var(key).with_context(|| format!("failed to load environment variable {}", key))
}
