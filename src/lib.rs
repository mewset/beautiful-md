//! beautiful-md: A CLI tool and library to format and beautify Markdown files.
//!
//! This crate provides functionality to parse, format, and beautify Markdown files
//! according to configurable style rules. It can be used as a library or as a
//! command-line tool.
//!
//! # Examples
//!
//! ```no_run
//! use beautiful_md::{Config, format_markdown};
//!
//! let markdown = "# Heading\n\n|Name|Age|\n|---|---|\n|Alice|30|";
//! let config = Config::default();
//! let formatted = format_markdown(markdown, &config).unwrap();
//! println!("{}", formatted);
//! ```
//!
//! # Features
//!
//! - Table alignment and padding
//! - Heading spacing normalization
//! - List indentation consistency
//! - Code block formatting
//! - Configurable via TOML files
//!
//! # Configuration
//!
//! Create a `.beautiful-md.toml` file in your project root or home directory:
//!
//! ```toml
//! [tables]
//! align = true
//! min_column_width = 3
//! padding = 1
//!
//! [headings]
//! blank_lines_before = 2
//! blank_lines_after = 1
//! space_after_hash = true
//!
//! [lists]
//! indent_size = 2
//! marker = "-"
//! normalize_numbers = true
//!
//! [code]
//! ensure_language_tag = false
//! fence_style = "```"
//! ```

#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod config;
pub mod error;
mod formatter;
mod formatters;

// Re-export main types for convenience
pub use config::Config;
pub use error::{Error, Result};

/// Format markdown content according to the provided configuration.
///
/// # Errors
///
/// Returns an error if the markdown cannot be parsed or formatted.
///
/// # Examples
///
/// ```
/// use beautiful_md::{Config, format_markdown};
///
/// let markdown = "# Heading\n\nSome text.";
/// let config = Config::default();
/// let result = format_markdown(markdown, &config);
/// assert!(result.is_ok());
/// ```
pub fn format_markdown(content: &str, config: &Config) -> Result<String> {
    formatter::format(content, config)
}

/// Format a markdown file in-place.
///
/// # Errors
///
/// Returns an error if the file cannot be read, parsed, formatted, or written.
pub fn format_file<P: AsRef<std::path::Path>>(path: P, config: &Config) -> Result<()> {
    let content = std::fs::read_to_string(path.as_ref())?;
    let formatted = format_markdown(&content, config)?;
    std::fs::write(path.as_ref(), formatted)?;
    Ok(())
}
