use std::{
    env::home_dir, fs, path::PathBuf
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub database_url: PathBuf,
}

impl Config {
    pub fn new() -> anyhow::Result<Config> {
        let database_url = get_db_path();

        Ok(Config { database_url })
    }
}

fn get_db_path() -> PathBuf {
    let home = home_dir().expect("Could not determine home directory");

    let data_dir = home.join(".local/share/duck-do");
    fs::create_dir_all(&data_dir)
        .expect("Failed to create duck-do data directory");

    let db_path = data_dir.join("client-db.sqlite");

    if !db_path.exists() {
        fs::write(&db_path, "")
            .expect("Failed to create database file");
        println!("Created initial database at {}", db_path.display());
    }

    db_path
}
