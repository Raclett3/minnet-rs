use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub host: String,
}

impl Config {
    pub fn from_file(path: &str) -> Option<Config> {
        let file = std::fs::read_to_string(path).ok()?;
        let config = toml::from_str(&file).ok()?;
        Some(config)
    }
}
