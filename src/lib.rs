#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    rust_2018_idioms,
    missing_debug_implementations,
    missing_docs
)]
#![allow(clippy::module_name_repetitions)] // Common in Rust APIs
#![allow(clippy::must_use_candidate)] // We'll add these selectively
#![allow(clippy::multiple_crate_versions)] // Transitive dependency conflicts we don't control

//! fsPrompt - A high-performance filesystem prompt generator for LLMs
//!
//! This library provides the core functionality for generating context prompts from codebases.

pub mod app;
pub mod core;
pub mod handlers;
pub mod state;
pub mod ui;
pub mod utils;
pub mod watcher;
/// Worker thread management for background tasks
pub mod workers;
