//! Error types for beautiful-md.
//!
//! This module defines all error types used throughout the library,
//! following Rust's error handling best practices.

use std::path::PathBuf;

/// Result type alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// The main error type for this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// I/O error occurred while reading or writing files.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Failed to parse markdown.
    #[error("Failed to parse markdown: {0}")]
    ParseError(String),

    /// Failed to load or parse configuration.
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// File path is invalid or inaccessible.
    #[error("Invalid file path: {}", .0.display())]
    InvalidPath(PathBuf),

    /// Pattern matching error for glob patterns.
    #[error("Pattern error: {0}")]
    PatternError(#[from] glob::PatternError),

    /// Glob iteration error.
    #[error("Glob error: {0}")]
    GlobError(#[from] glob::GlobError),

    /// TOML deserialization error.
    #[error("TOML error: {0}")]
    TomlError(#[from] toml::de::Error),

    /// Custom error for formatting issues.
    #[error("Formatting error: {0}")]
    FormattingError(String),
}
