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

    // Apply formatters (code blocks are already protected at this point)
    result = format_tables(&result, &config.tables);
    result = format_headings(&result, &config.headings);
    result = format_lists(&result, &config.lists);

    result
}

/// Extract code blocks from content early (before parsing), replacing them with placeholders.
///
/// This preserves code blocks completely verbatim, preventing any markdown processing.
/// Returns the content with placeholders and a vec of extracted code blocks.
pub fn extract_code_blocks_early(content: &str) -> (String, Vec<(String, String)>) {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut code_blocks = Vec::new();
    let mut in_code_block = false;
    let mut current_block = Vec::new();
    let mut current_lang = String::new();

    for line in lines {
        let trimmed = line.trim();

        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            if in_code_block {
                // End of code block
                code_blocks.push((current_lang.clone(), current_block.join("\n")));
                result.push(format!(
                    "<!--BEAUTIFUL_MD_CODE_BLOCK_{}-->",
                    code_blocks.len() - 1
                ));
                current_block.clear();
                in_code_block = false;
            } else {
                // Start of code block
                in_code_block = true;
                current_lang = if trimmed.len() > 3 {
                    trimmed[3..].trim().to_string()
                } else {
                    String::new()
                };
            }
        } else if in_code_block {
            current_block.push(line);
        } else {
            result.push(line.to_string());
        }
    }

    // Handle unclosed code block
    if in_code_block {
        code_blocks.push((current_lang, current_block.join("\n")));
        result.push(format!(
            "<!--BEAUTIFUL_MD_CODE_BLOCK_{}-->",
            code_blocks.len() - 1
        ));
    }

    (result.join("\n"), code_blocks)
}

/// Restore code blocks into content early (after all formatting), replacing placeholders.
///
/// Applies the configured fence style while preserving code block content verbatim.
pub fn restore_code_blocks_early(
    content: &str,
    code_blocks: &[(String, String)],
    config: &Config,
) -> String {
    let fence = &config.code.fence_style;
    let mut result = content.to_string();

    for (i, (lang, block_content)) in code_blocks.iter().enumerate() {
        let placeholder = format!("<!--BEAUTIFUL_MD_CODE_BLOCK_{i}-->");
        let code_block = if lang.is_empty() {
            format!("{fence}\n{block_content}\n{fence}")
        } else {
            format!("{fence}{lang}\n{block_content}\n{fence}")
        };

        result = result.replace(&placeholder, &code_block);
    }

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
