//! Configuration management for beautiful-md.
//!
//! This module handles loading and parsing configuration from TOML files,
//! with sensible defaults for all options.

use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::error::{Error, Result};

/// Main configuration structure.
///
/// Can be loaded from a TOML file or created with defaults.
///
/// # Example
///
/// ```no_run
/// use beautiful_md::Config;
///
/// // Load from file
/// let config = Config::from_file(".beautiful-md.toml").unwrap();
///
/// // Or use defaults
/// let config = Config::default();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    /// Table formatting options.
    pub tables: TableConfig,

    /// Heading formatting options.
    pub headings: HeadingConfig,

    /// List formatting options.
    pub lists: ListConfig,

    /// Code block formatting options.
    pub code: CodeConfig,
}

/// Configuration for table formatting.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TableConfig {
    /// Enable column alignment.
    pub align: bool,

    /// Minimum column width in characters.
    pub min_column_width: usize,

    /// Padding around cell content.
    pub padding: usize,
}

/// Configuration for heading formatting.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct HeadingConfig {
    /// Number of blank lines before headings.
    pub blank_lines_before: usize,

    /// Number of blank lines after headings.
    pub blank_lines_after: usize,

    /// Ensure space after `#` symbols.
    pub space_after_hash: bool,
}

/// Configuration for list formatting.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ListConfig {
    /// Indentation size in spaces.
    pub indent_size: usize,

    /// Bullet marker character (`-`, `*`, or `+`).
    pub marker: String,

    /// Normalize ordered list numbers (1, 2, 3... vs 1, 1, 1...).
    pub normalize_numbers: bool,
}

/// Configuration for code block formatting.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CodeConfig {
    /// Ensure language tags are present.
    pub ensure_language_tag: bool,

    /// Code fence style (` ``` ` or `~~~`).
    pub fence_style: String,
}

impl Default for TableConfig {
    fn default() -> Self {
        Self {
            align: true,
            min_column_width: 3,
            padding: 1,
        }
    }
}

impl Default for HeadingConfig {
    fn default() -> Self {
        Self {
            blank_lines_before: 2,
            blank_lines_after: 1,
            space_after_hash: true,
        }
    }
}

impl Default for ListConfig {
    fn default() -> Self {
        Self {
            indent_size: 2,
            marker: String::from("-"),
            normalize_numbers: true,
        }
    }
}

impl Default for CodeConfig {
    fn default() -> Self {
        Self {
            ensure_language_tag: false,
            fence_style: String::from("```"),
        }
    }
}

impl Config {
    /// Load configuration from a TOML file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path.as_ref()).map_err(Error::Io)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }

    /// Try to load configuration from default locations.
    ///
    /// Searches for `.beautiful-md.toml` in:
    /// 1. Current directory
    /// 2. User's home directory
    ///
    /// If no config file is found, returns default configuration.
    #[must_use]
    pub fn load_default() -> Self {
        // Try current directory
        if let Ok(config) = Self::from_file(".beautiful-md.toml") {
            return config;
        }

        // Try home directory
        if let Some(home) = dirs::home_dir() {
            let home_config = home.join(".beautiful-md.toml");
            if let Ok(config) = Self::from_file(home_config) {
                return config;
            }
        }

        // Fall back to defaults
        Self::default()
    }

    /// Save configuration to a TOML file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written.
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| Error::ConfigError(format!("Failed to serialize config: {e}")))?;
        std::fs::write(path.as_ref(), content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.tables.align);
        assert_eq!(config.tables.min_column_width, 3);
        assert_eq!(config.headings.blank_lines_before, 2);
        assert_eq!(config.lists.marker, "-");
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml = toml::to_string(&config).unwrap();
        let parsed: Config = toml::from_str(&toml).unwrap();
        assert_eq!(parsed.tables.align, config.tables.align);
    }
}
