//! Configuration persistence for fsPrompt

use crate::core::types::{AppConfig, Theme};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Serializable configuration for persistence
/// This is a separate type to maintain backward compatibility
/// and handle legacy config migrations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableConfig {
    /// Window dimensions
    pub window_width: f32,
    /// Window height in pixels
    pub window_height: f32,

    /// Split position (0.0 to 1.0)
    pub split_position: f32,

    /// Last opened directory
    pub last_directory: Option<PathBuf>,

    /// Ignore patterns (comma-separated)
    pub ignore_patterns: String,

    /// Include directory tree in output
    pub include_tree: bool,

    /// Last used output format
    pub output_format: String,

    /// Theme preference: "auto", "light", "dark"
    pub theme: String,
}

impl Default for SerializableConfig {
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

impl From<&AppConfig> for SerializableConfig {
    fn from(config: &AppConfig) -> Self {
        Self {
            window_width: config.window.width,
            window_height: config.window.height,
            split_position: config.window.left_pane_ratio,
            last_directory: None, // This should come from app state
            ignore_patterns: config.ignore_patterns.join(","),
            include_tree: true,               // This should come from output state
            output_format: "xml".to_string(), // This should come from output state
            theme: match config.ui.theme {
                Theme::Light => "light".to_string(),
                Theme::Dark => "dark".to_string(),
                Theme::System => "auto".to_string(),
            },
        }
    }
}

impl SerializableConfig {
    /// Convert to AppConfig with defaults for missing fields
    pub fn to_app_config(&self) -> AppConfig {
        crate::core::types::AppConfig {
            window: crate::core::types::WindowConfig {
                width: self.window_width,
                height: self.window_height,
                left_pane_ratio: self.split_position,
            },
            ui: crate::core::types::UiConfig {
                theme: match self.theme.as_str() {
                    "light" => Theme::Light,
                    "dark" => Theme::Dark,
                    _ => Theme::System,
                },
                font_size: 14.0,    // Default
                show_hidden: false, // Default
                include_tree: self.include_tree,
            },
            ignore_patterns: if self.ignore_patterns.is_empty() {
                Vec::new()
            } else {
                self.ignore_patterns
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect()
            },
            performance: crate::core::types::PerformanceConfig {
                max_concurrent_reads: 10, // Default
                cache_size_mb: 100,       // Default
                use_mmap: true,           // Default
            },
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
            Ok(content) => {
                if let Ok(serializable) = serde_json::from_str::<SerializableConfig>(&content) {
                    serializable.to_app_config()
                } else {
                    AppConfig::default()
                }
            }
            Err(_) => AppConfig::default(),
        }
    }

    /// Save configuration to disk
    pub fn save(&self, config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
        let serializable = SerializableConfig::from(config);
        let json = serde_json::to_string_pretty(&serializable)?;
        std::fs::write(&self.config_path, json)?;
        Ok(())
    }
}
