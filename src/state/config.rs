//! Configuration persistence for fsPrompt

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application configuration for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Window dimensions
    pub window_width: f32,
    /// Window height in pixels
    pub window_height: f32,

    /// Split position (0.0 to 1.0)
    pub split_position: f32,

    /// Last opened directory
    pub last_directory: Option<PathBuf>,

    /// Ignore patterns
    pub ignore_patterns: String,

    /// Include directory tree in output
    pub include_tree: bool,

    /// Last used output format
    pub output_format: String,

    /// Theme preference: "auto", "light", "dark"
    pub theme: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window_width: 1200.0,
            window_height: 800.0,
            split_position: 0.3,
            last_directory: None,
            ignore_patterns: String::new(),
            include_tree: true,
            output_format: "xml".to_string(),
            theme: "auto".to_string(),
        }
    }
}

/// Manages loading and saving of application configuration
#[derive(Debug)]
pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    /// Creates a new config manager with platform-specific config path
    pub fn new() -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("fsprompt");

        // Ensure config directory exists
        let _ = std::fs::create_dir_all(&config_dir);

        Self {
            config_path: config_dir.join("config.json"),
        }
    }

    /// Load configuration from disk, returns default if not found or invalid
    pub fn load(&self) -> AppConfig {
        match std::fs::read_to_string(&self.config_path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => AppConfig::default(),
        }
    }

    /// Save configuration to disk
    pub fn save(&self, config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(config)?;
        std::fs::write(&self.config_path, json)?;
        Ok(())
    }
}
