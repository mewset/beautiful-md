//! Individual markdown formatters.
//!
//! This module contains specialized formatters for different markdown elements:
//! - Tables
//! - Headings
//! - Lists
//! - Code blocks

mod code;
mod heading;
mod list;
mod table;

use code::format_code_blocks;
use heading::format_headings;
use list::format_lists;
use table::format_tables;

use crate::config::Config;

/// Apply all formatters to markdown content.
///
/// This function orchestrates the application of all individual formatters
/// in the correct order to avoid conflicts.
pub fn apply_all(content: &str, config: &Config) -> String {
    let mut result = content.to_string();

    // Apply formatters in order of least to most invasive
    result = format_code_blocks(&result, &config.code);
    result = format_tables(&result, &config.tables);
    result = format_headings(&result, &config.headings);
    result = format_lists(&result, &config.lists);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_all() {
        let content = "# Test\n\nSome text.";
        let config = Config::default();
        let result = apply_all(content, &config);
        assert!(!result.is_empty());
    }
}
