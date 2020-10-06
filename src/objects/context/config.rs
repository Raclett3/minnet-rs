use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub database: Database,
}

impl Config {
    pub fn from_file(path: &str) -> Option<Config> {
        let file = std::fs::read_to_string(path).ok()?;
        let config = toml::from_str(&file).ok()?;
        Some(config)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub host: String,
    pub user: String,
    pub password: String,
    pub database: String,
}
