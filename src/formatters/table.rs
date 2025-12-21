//! Table formatting module.
//!
//! Handles alignment, padding, and beautification of Markdown tables.

#![allow(clippy::format_push_string)]
#![allow(clippy::uninlined_format_args)]

use crate::config::TableConfig;

/// Format tables in markdown content.
///
/// Aligns columns, adds padding, and ensures consistent spacing.
pub fn format_tables(content: &str, config: &TableConfig) -> String {
    if !config.align {
        return content.to_string();
    }

    let mut result = String::new();
    let mut in_table = false;
    let mut table_lines = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Detect table lines (contains pipes but not in code blocks)
        if trimmed.starts_with('|') || trimmed.contains('|') {
            if !in_table {
                in_table = true;
                table_lines.clear();
            }
            table_lines.push(line.to_string());
        } else if in_table {
            // End of table, format and add
            if !table_lines.is_empty() {
                result.push_str(&format_table(&table_lines, config));
            }
            table_lines.clear();
            in_table = false;
            result.push_str(line);
            result.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    // Handle table at end of file
    if !table_lines.is_empty() {
        result.push_str(&format_table(&table_lines, config));
    }

    result
}

/// Format a single table.
fn format_table(lines: &[String], config: &TableConfig) -> String {
    if lines.len() < 2 {
        // Not a valid table
        return lines.join("\n") + "\n";
    }

    // Parse table rows
    let rows: Vec<Vec<String>> = lines
        .iter()
        .map(|line| {
            line.split('|')
                .map(|cell| cell.trim().to_string())
                .filter(|cell| !cell.is_empty())
                .collect()
        })
        .collect();

    if rows.is_empty() {
        return lines.join("\n") + "\n";
    }

    // Calculate column widths
    let num_cols = rows.iter().map(Vec::len).max().unwrap_or(0);
    let mut col_widths = vec![config.min_column_width; num_cols];

    for row in &rows {
        for (i, cell) in row.iter().enumerate() {
            // Skip separator rows
            if !cell.chars().all(|c| c == '-' || c == ':') {
                col_widths[i] = col_widths[i].max(cell.len());
            }
        }
    }

    // Format rows
    let mut formatted = String::new();
    for row in &rows {
        formatted.push('|');

        for (col_idx, cell) in row.iter().enumerate() {
            let width = col_widths[col_idx];
            let padding = " ".repeat(config.padding);

            // Check if this is a separator row
            if cell.chars().all(|c| c == '-' || c == ':') {
                // Preserve alignment indicators
                let sep = if cell.starts_with(':') && cell.ends_with(':') {
                    format!(":{:-<width$}:", "", width = width - 2)
                } else if cell.ends_with(':') {
                    format!("{:-<width$}:", "", width = width - 1)
                } else if cell.starts_with(':') {
                    format!(":{:-<width$}", "", width = width - 1)
                } else {
                    "-".repeat(width)
                };
                formatted.push_str(&format!("{padding}{sep}{padding}|"));
            } else {
                // Regular cell
                formatted.push_str(&format!(
                    "{padding}{:<width$}{padding}|",
                    cell,
                    width = width
                ));
            }
        }

        formatted.push('\n');
    }

    formatted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_simple_table() {
        let input = "|Name|Age|\n|---|---|\n|Alice|30|";
        let config = TableConfig::default();
        let result = format_tables(input, &config);

        assert!(result.contains("Name"));
        assert!(result.contains("Alice"));
    }

    #[test]
    fn test_format_table_disabled() {
        let input = "|Name|Age|\n|---|---|\n|Alice|30|";
        let config = TableConfig {
            align: false,
            ..Default::default()
        };

        let result = format_tables(input, &config);
        assert_eq!(result, input);
    }
}
