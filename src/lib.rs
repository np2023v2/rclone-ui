//! Rclone UI - A TUI for rclone cloud file management
//!
//! This application provides a terminal user interface (TUI) for managing
//! cloud files using rclone.

pub mod rclone;
pub mod tui;

pub use rclone::*;

/// Application result type
pub type Result<T> = anyhow::Result<T>;

/// Application configuration
#[derive(Debug, Clone, Default)]
pub struct Config {
    /// Rclone configuration path (optional)
    pub rclone_config: Option<String>,
}
