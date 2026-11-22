use std::{
    fs::{DirBuilder, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::error::ApplicationError;

/**
 * The settings for the application.
 */
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
    #[must_use]
    pub fn new(apinae_path: Option<String>) -> Self {
        Self { apinae_path }
    }

    /**
     * Load settings from a file.
     *
     * # Returns
     * Settings - The settings object. If any error occurs then the default settings will be returned.
     */
    #[must_use]
    pub fn load() -> Self {
        match dirs::home_dir() {
            Some(dir) => {
                let file_path = Self::settings_path_from_home(&dir);
                Self::load_from_path(&file_path)
            }
            None => Settings::default(),
        }
    }

    /**
     * Save the settings to a file.
     *
     * # Returns
     * The result of saving the settings.
     *
     * # Errors
     * If any error occurs while saving the settings, an `ApplicationError` will be returned.
     */
    pub fn save(&self) -> Result<(), ApplicationError> {
        let home_dir = dirs::home_dir().ok_or_else(|| ApplicationError::ConfigurationError("Could not find home directory".to_string()))?;
        let file_path = Self::settings_path_from_home(&home_dir);
        self.save_to_path(&file_path)
    }

    /**
     * Get the path to the settings file from the home directory.   
     *
     * # Arguments
     * * `home_dir` - The home directory path.
     *
     * # Returns
     * The path to the settings file.
     */
    fn settings_path_from_home(home_dir: &Path) -> PathBuf {
        home_dir.join(".config/apinae/settings.json")
    }

    /**
     * Load settings from a file.
     *
     * # Arguments
     * `file_path` - The path to the settings file.
     *
     * # Returns
     *  The settings object. If any error occurs then the default settings will be returned.
     */
    fn load_from_path(file_path: &Path) -> Self {
        std::fs::read_to_string(file_path).ok().and_then(|contents| serde_json::from_str(&contents).ok()).unwrap_or_default()
    }

    /**
     * Save the settings to a file.
     *
     * # Arguments
     * `file_path` - The path to the settings file.
     *
     * # Returns
     * The result of saving the settings.
     *
     * # Errors
     * If any error occurs while saving the settings, an `ApplicationError` will be returned.
     */
    fn save_to_path(&self, file_path: &Path) -> Result<(), ApplicationError> {
        let settings = serde_json::to_string(self).map_err(|err| ApplicationError::ConfigurationError(err.to_string()))?;

        if let Some(config_dir) = file_path.parent() {
            DirBuilder::new().recursive(true).create(config_dir).map_err(|err| ApplicationError::FileError(err.to_string()))?;
        }

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

#[cfg(test)]
mod test {
    use super::*;

    fn temp_path(name: &str) -> PathBuf {
        let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        std::env::temp_dir().join(format!("apinae_settings_{name}_{}_{}.json", std::process::id(), nanos))
    }

    #[test]
    fn new_and_default_settings() {
        let settings = Settings::new(Some("/opt/apinae".to_string()));
        assert_eq!(settings.apinae_path.as_deref(), Some("/opt/apinae"));

        assert_eq!(Settings::default(), Settings::new(None));
        assert_eq!(Settings::default().apinae_path, None);
    }

    #[test]
    fn save_and_load_from_path() {
        let path = temp_path("save_load");
        let settings = Settings::new(Some("/usr/local/bin/apinae".to_string()));

        settings.save_to_path(&path).unwrap();
        let loaded = Settings::load_from_path(&path);

        assert_eq!(loaded, settings);

        let contents = std::fs::read_to_string(&path).unwrap();
        assert_eq!(contents, r#"{"apinaePath":"/usr/local/bin/apinae"}"#);

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn load_from_missing_or_invalid_path_returns_default() {
        let missing = temp_path("missing");
        assert_eq!(Settings::load_from_path(&missing), Settings::default());

        std::fs::write(&missing, "not valid json").unwrap();
        assert_eq!(Settings::load_from_path(&missing), Settings::default());

        std::fs::write(&missing, r#"{"unknownField":"value"}"#).unwrap();
        assert_eq!(Settings::load_from_path(&missing), Settings::default());

        let _ = std::fs::remove_file(missing);
    }

    #[test]
    fn save_to_nested_directory_creates_directory() {
        let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let directory = std::env::temp_dir().join(format!("apinae_settings_dir_{}_{}", std::process::id(), nanos));
        let path = directory.join("nested/settings.json");

        Settings::new(Some("apinae".to_string())).save_to_path(&path).unwrap();
        assert!(path.is_file());

        let _ = std::fs::remove_dir_all(directory);
    }

    #[test]
    fn save_to_invalid_directory_returns_file_error() {
        let invalid_parent = temp_path("parent_file");
        std::fs::write(&invalid_parent, "not a directory").unwrap();

        let path = invalid_parent.join("settings.json");
        let result = Settings::new(Some("apinae".to_string())).save_to_path(&path);

        assert!(matches!(result, Err(ApplicationError::FileError(_))));

        let _ = std::fs::remove_file(invalid_parent);
    }

    #[test]
    fn settings_path_is_under_config_directory() {
        let home = std::env::temp_dir().join("apinae_home");
        let path = Settings::settings_path_from_home(&home);
        assert_eq!(path, home.join(".config/apinae/settings.json"));
    }
}
