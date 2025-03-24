use std::{
    fs::{DirBuilder, OpenOptions},
    io::Write,
};

use serde::{Deserialize, Serialize};

use crate::error::ApplicationError;

/**
 * The settings for the application.
 */
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    // Use specific apinae command. If none then use the default apinae command will be used.
    pub apinae_path: Option<String>,
}

impl Settings {
    /** 3
     * Create a new settings object.
     *
     * # Arguments
     * `apinae_path`: Option<String> - The path to the apinae command.
     *
     * # Returns
     * Settings - The settings object.
     */
    pub fn new(apinae_path: Option<String>) -> Self {
        Self { apinae_path }
    }

    /**
     * Load settings from a file.
     *  
     * # Returns
     * Settings - The settings object. If any error occurs then the default settings will be returned.
     */
    pub fn load() -> Self {
        match dirs::home_dir() {
            Some(dir) => {
                let config_dir = dir.join(".config/apinae");
                let file_path = config_dir.join("settings.json");
                std::fs::read_to_string(file_path).map(|str| serde_json::from_str(&str).unwrap_or_default()).unwrap_or_default()
            }
            None => Settings::default(),
        }
    }

    /**
     * Save the settings to a file.
     *
     * # Arguments
     * `self` - The settings object.
     *
     * # Returns
     * Result<(), ApplicationError> - The result of saving the settings.
     */
    pub fn save(&self) -> Result<(), ApplicationError> {
        let home_dir = dirs::home_dir().ok_or(ApplicationError::ConfigurationError("Could not find home directory".to_string()))?;
        let config_dir = home_dir.join(".config/apinae");
        let file_path = config_dir.join("settings.json");
        let settings = serde_json::to_string(&self).map_err(|err| ApplicationError::ConfigurationError(err.to_string()))?;
        let _ = DirBuilder::new().recursive(true).create(config_dir).map_err(|err| ApplicationError::FileError(err.to_string()));
        let mut file = OpenOptions::new().truncate(true).write(true).create(true).open(file_path).map_err(|err| ApplicationError::FileError(err.to_string()))?;
        file.write_all(settings.as_bytes()).map_err(|err| ApplicationError::FileError(err.to_string()))
    }
}

impl Default for Settings {
    /**
     * Default settings for the application.
     */
    fn default() -> Self {
        Self { apinae_path: None }
    }
}
