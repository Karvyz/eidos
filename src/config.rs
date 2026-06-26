use std::path::PathBuf;

use serde::Deserialize;

/// Eidos configuration loaded from XDG config path.
#[derive(Deserialize, Debug)]
pub struct Config {
    /// Path to the notes vault directory.
    pub vault_path: Option<PathBuf>,
}

impl Config {
    /// Load config from `$XDG_CONFIG_HOME/eidos/config.toml`.
    /// Returns `None` if the file does not exist or cannot be parsed.
    pub fn load() -> Option<Self> {
        let config_dir = dirs::config_dir()?;
        let config_path = config_dir.join("eidos").join("config.toml");

        let contents = std::fs::read_to_string(&config_path).ok()?;
        toml::from_str(&contents).ok()
    }
}