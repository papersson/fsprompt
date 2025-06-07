//! State management and persistence for fsPrompt

pub mod config;
pub mod history;

pub use config::{AppConfig, ConfigManager};
pub use history::{HistoryManager, SelectionSnapshot};
