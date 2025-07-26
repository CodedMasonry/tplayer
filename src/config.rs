use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    sync::OnceLock,
};

use serde::{Deserialize, Serialize};

/*
 * Statics
 */

/// Config path so it can be referenced later
static CONFIG_PATH: OnceLock<PathBuf> = OnceLock::new();

/*
 * Data
 */

/// Global Config
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub volume: f32,
}

impl Config {
    /*
     * Init
     */

    /// Creates a new config and saves it
    fn create_new(path: &Path) -> Self {
        let default = Config::default();
        let serialized = serde_json::to_string(&default).unwrap();

        // Expect permissions allow for saving config at path
        fs::create_dir_all(path.parent().unwrap()).expect("Failed to generate path");
        fs::write(path, serialized)
            .expect(&format!("Failed to save config to `{}`", path.display()));

        return default;
    }

    /// Try to parse config, else create a new one
    pub fn parse_or_new(path: &Path) -> Self {
        // Try to open file, creates a new one if not possible
        let file = match File::open(path) {
            Ok(v) => v,
            Err(_) => {
                return Config::create_new(path);
            }
        };

        // For now, if we fail to parse the config create a new one
        let conf: Config = match serde_json::from_reader(file) {
            Ok(v) => v,
            Err(_) => {
                return Config::create_new(path);
            }
        };

        // Save path for later
        CONFIG_PATH.set(path.to_path_buf()).unwrap();

        return conf;
    }

    /*
     * Updating Config
     */

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
        self.save();
    }

    /*
     * Cleanup
     */

    /// Saves config to main path
    pub fn save(&self) {
        let path = CONFIG_PATH.get().unwrap();
        let serialized = serde_json::to_string(self).unwrap();

        fs::write(path, serialized).expect("Failed to save config")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self { volume: 1.0 }
    }
}
