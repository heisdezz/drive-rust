use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::theme::ThemeMode;

/// Default version identifier for configuration schema migrations.
fn default_version() -> u32 {
    1
}

/// Default albums directory relative to drive root.
fn default_albums_dir() -> String {
    "albums".to_string()
}

/// Default catalog grid columns count.
fn default_grid_cols() -> usize {
    6
}

/// Configuration settings specific to a storage volume or drive.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DriveConfig {
    /// Default location of albums for this drive.
    /// Can be a relative directory name (e.g. "albums", "Photos", "Pictures/Albums")
    /// or an absolute directory path.
    #[serde(default = "default_albums_dir")]
    pub albums_dir: String,

    /// Optional custom SQLite database / library path override.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_library_path: Option<PathBuf>,
}

impl Default for DriveConfig {
    fn default() -> Self {
        Self {
            albums_dir: default_albums_dir(),
            custom_library_path: None,
        }
    }
}

/// General application-wide settings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneralConfig {
    /// Active theme mode.
    #[serde(default)]
    pub theme: ThemeMode,

    /// Number of columns in the media catalog grid (4 to 9).
    #[serde(default = "default_grid_cols")]
    pub grid_columns: usize,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            theme: ThemeMode::ZedDark,
            grid_columns: default_grid_cols(),
        }
    }
}

/// The top-level application configuration persisted in `config.json`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    #[serde(default = "default_version")]
    pub version: u32,

    #[serde(default)]
    pub general: GeneralConfig,

    /// Per-drive configurations, keyed by mount path (e.g. "/run/media/destiny/Ventoy")
    /// or volume identifier.
    #[serde(default)]
    pub drives: HashMap<String, DriveConfig>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: 1,
            general: GeneralConfig::default(),
            drives: HashMap::new(),
        }
    }
}

impl AppConfig {
    /// Resolves the absolute path to the JSON configuration file.
    /// Checks `RUST_DRIVE_CONFIG_PATH`, then `$XDG_CONFIG_HOME/rust-drive/config.json`,
    /// then `$HOME/.config/rust-drive/config.json`.
    pub fn config_path() -> PathBuf {
        if let Ok(custom) = std::env::var("RUST_DRIVE_CONFIG_PATH") {
            return PathBuf::from(custom);
        }
        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            return PathBuf::from(xdg).join("rust-drive").join("config.json");
        }
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home).join(".config").join("rust-drive").join("config.json");
        }
        PathBuf::from(".config/rust-drive/config.json")
    }

    /// Loads configuration from the JSON file on disk, returning default on missing/corrupt.
    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(config) = serde_json::from_str::<AppConfig>(&content) {
                    return config;
                }
            }
        }
        Self::default()
    }

    /// Persists configuration to disk formatted as human-readable JSON.
    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Serialization error: {}", e))?;
        fs::write(&path, json)
            .map_err(|e| format!("Failed to write config file to {}: {}", path.display(), e))
    }

    /// Retrieves settings for a specific drive, matching by mount path, drive id, or drive name.
    pub fn get_drive_config(&self, mount_path: &str, drive_id: &str, drive_name: &str) -> DriveConfig {
        if let Some(cfg) = self.drives.get(mount_path) {
            return cfg.clone();
        }
        if let Some(cfg) = self.drives.get(drive_id) {
            return cfg.clone();
        }
        if let Some(cfg) = self.drives.get(drive_name) {
            return cfg.clone();
        }
        DriveConfig::default()
    }

    /// Sets or updates the settings for a specific drive key.
    pub fn set_drive_config(&mut self, key: &str, config: DriveConfig) {
        self.drives.insert(key.to_string(), config);
    }

    /// Sets the albums directory for a drive.
    pub fn set_drive_albums_dir(&mut self, key: &str, albums_dir: &str) {
        let mut cfg = self.drives.get(key).cloned().unwrap_or_default();
        cfg.albums_dir = if albums_dir.trim().is_empty() {
            default_albums_dir()
        } else {
            albums_dir.trim().to_string()
        };
        self.drives.insert(key.to_string(), cfg);
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_default_config_values() {
        let config = AppConfig::default();
        assert_eq!(config.version, 1);
        assert_eq!(config.general.theme, ThemeMode::ZedDark);
        assert_eq!(config.general.grid_columns, 6);
        assert!(config.drives.is_empty());
    }

    #[test]
    fn test_drive_config_fallback() {
        let config = AppConfig::default();
        let drive_cfg = config.get_drive_config("/media/usb", "usb_1", "MyUSB");
        assert_eq!(drive_cfg.albums_dir, "albums");
        assert_eq!(drive_cfg.custom_library_path, None);
    }

    #[test]
    fn test_per_drive_settings_persistence() {
        let temp = tempdir().unwrap();
        let config_file = temp.path().join("test_config.json");
        std::env::set_var("RUST_DRIVE_CONFIG_PATH", config_file.to_str().unwrap());

        let mut config = AppConfig::load();
        assert_eq!(config.drives.len(), 0);

        // Configure Drive 1 with custom albums folder
        let drive1_key = "/run/media/destiny/Ventoy";
        config.set_drive_albums_dir(drive1_key, "CustomPhotos/Albums");
        assert!(config.save().is_ok());

        // Reload from file
        let reloaded = AppConfig::load();
        let d1_cfg = reloaded.get_drive_config(drive1_key, "id1", "Ventoy");
        assert_eq!(d1_cfg.albums_dir, "CustomPhotos/Albums");

        // Drive 2 still falls back to default "albums"
        let d2_cfg = reloaded.get_drive_config("/run/media/destiny/SDCard", "id2", "SDCard");
        assert_eq!(d2_cfg.albums_dir, "albums");

        std::env::remove_var("RUST_DRIVE_CONFIG_PATH");
    }

    #[test]
    fn test_theme_mode_serialization() {
        let mut gen = GeneralConfig::default();
        gen.theme = ThemeMode::Nord;
        assert_eq!(gen.theme, ThemeMode::Nord);
    }
}
