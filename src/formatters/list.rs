//! List formatting module.
//!
//! Handles indentation and marker consistency for Markdown lists.

use crate::config::ListConfig;

/// Format lists in markdown content.
///
/// Normalizes indentation and list markers according to configuration.
pub fn format_lists(content: &str, config: &ListConfig) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut ordered_counter = 1;
    let mut in_ordered_list = false;

    for line in lines {
        let trimmed = line.trim();

        if is_unordered_list_item(trimmed) {
            in_ordered_list = false;
            let (level, content) = parse_list_item(line);
            let indent = " ".repeat(level * config.indent_size);
            result.push(format!("{indent}{} {content}", config.marker));
        } else if is_ordered_list_item(trimmed) {
            if !in_ordered_list {
                ordered_counter = 1;
                in_ordered_list = true;
            }

            let (level, content) = parse_list_item(line);
            let indent = " ".repeat(level * config.indent_size);

            if config.normalize_numbers {
                result.push(format!("{indent}{ordered_counter}. {content}"));
                ordered_counter += 1;
            } else {
                // Keep original numbering
                result.push(line.to_string());
            }
        } else {
            // Reset ordered list counter on non-list lines
            if !trimmed.is_empty() {
                in_ordered_list = false;
                ordered_counter = 1;
            }
            result.push(line.to_string());
        }
    }

    result.join("\n")
}

/// Check if a line is an unordered list item.
fn is_unordered_list_item(line: &str) -> bool {
    let trimmed = line.trim();
    (trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ "))
        && !trimmed.starts_with("---")
}

/// Check if a line is an ordered list item.
fn is_ordered_list_item(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.chars().next().is_some_and(|c| c.is_ascii_digit()) && trimmed.contains(". ")
}

/// Parse list item to determine indentation level and content.
fn parse_list_item(line: &str) -> (usize, String) {
    let leading_spaces = line.len() - line.trim_start().len();
    let level = leading_spaces / 2; // Assume 2-space indents in input

    let trimmed = line.trim();
    // Remove marker and number
    let content = if is_unordered_list_item(trimmed) {
        trimmed[2..].to_string()
    } else if is_ordered_list_item(trimmed) {
        // Find the period and skip number
        trimmed
            .find(". ")
            .map_or_else(|| trimmed.to_string(), |pos| trimmed[pos + 2..].to_string())
    } else {
        trimmed.to_string()
    };

    (level, content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_unordered_list_item() {
        assert!(is_unordered_list_item("- Item"));
        assert!(is_unordered_list_item("* Item"));
        assert!(is_unordered_list_item("+ Item"));
        assert!(!is_unordered_list_item("Not a list"));
    }

    #[test]
    fn test_is_ordered_list_item() {
        assert!(is_ordered_list_item("1. Item"));
        assert!(is_ordered_list_item("42. Item"));
        assert!(!is_ordered_list_item("Not a list"));
    }

    #[test]
    fn test_parse_list_item() {
        let (level, content) = parse_list_item("- Item");
        assert_eq!(level, 0);
        assert_eq!(content, "Item");

        let (level, content) = parse_list_item("  - Nested");
        assert_eq!(level, 1);
        assert_eq!(content, "Nested");
    }

    #[test]
    fn test_format_lists() {
        let input = "- Item 1\n* Item 2\n+ Item 3";
        let config = ListConfig {
            indent_size: 2,
            marker: String::from("-"),
            normalize_numbers: true,
        };

        let result = format_lists(input, &config);
        assert!(result.contains("- Item 1"));
        assert!(result.contains("- Item 2"));
        assert!(result.contains("- Item 3"));
    }

    #[test]
    fn test_bold_before_list() {
        // This is what cmark() produces
        let input = "# Test\n\n**Table of Contents:**\n\n* Tables with various structures\n* Headings at different levels";
        let config = ListConfig {
            indent_size: 2,
            marker: String::from("-"),
            normalize_numbers: true,
        };

        let result = format_lists(input, &config);
        eprintln!("INPUT:\n{input}");
        eprintln!("\nOUTPUT:\n{result}");

        // Bold text should NOT be converted to list item
        assert!(result.contains("**Table of Contents:**"));
        assert!(!result.contains("- *Table of Contents"));
    }
}
