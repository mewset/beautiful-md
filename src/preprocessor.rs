//! Pre-processing module to fix common markdown issues before parsing.
//!
//! This module intelligently fixes malformed markdown so it can be properly
//! parsed and formatted, rather than being escaped or ignored.

use crate::diagnostics::{Diagnostic, DiagnosticKind, Diagnostics, Severity};

/// Pre-process markdown content to fix common issues.
///
/// Returns the preprocessed content and any diagnostics collected.
pub fn preprocess(content: &str) -> (String, Diagnostics) {
    let mut diagnostics = Diagnostics::new();
    let mut result = content.to_string();

    // Apply pre-processors in order
    result = fix_headings(&result);
    result = fix_list_markers(&result);
    result = fix_table_pipes(&result, &mut diagnostics);

    (result, diagnostics)
}

/// Fix heading syntax issues.
///
/// Fixes:
/// - `#NoSpace` → `# NoSpace`
/// - `####Trailing####` → `#### Trailing`
/// - `###  TooManySpaces` → `### TooManySpaces`
fn fix_headings(content: &str) -> String {
    let lines: Vec<String> = content
        .lines()
        .map(|line| {
            let trimmed = line.trim();

            // Check if line starts with hashes
            if trimmed.starts_with('#') && !trimmed.starts_with("```") {
                let mut chars = trimmed.chars();
                let mut hashes = String::new();

                // Collect leading hashes
                for ch in chars.by_ref() {
                    if ch == '#' {
                        hashes.push(ch);
                        if hashes.len() > 6 {
                            // Too many hashes, not a valid heading
                            return line.to_string();
                        }
                    } else {
                        // Found first non-hash character
                        let rest = format!("{}{}", ch, chars.as_str());
                        let rest_trimmed = rest.trim();

                        if rest_trimmed.is_empty() {
                            // Only hashes, not valid
                            return line.to_string();
                        }

                        // Remove trailing hashes
                        let mut content = rest_trimmed.to_string();
                        while content.ends_with('#') {
                            content.pop();
                        }
                        let content = content.trim_end();

                        // Reconstruct heading with proper spacing
                        return if ch.is_whitespace() {
                            // Already has space after hashes, just normalize
                            let text = rest_trimmed.trim_end_matches('#').trim();
                            format!("{hashes} {text}")
                        } else {
                            // No space after hashes, add it
                            format!("{hashes} {content}")
                        };
                    }
                }
            }

            line.to_string()
        })
        .collect();

    lines.join("\n")
}

/// Fix list marker inconsistencies.
///
/// Normalizes list markers while preserving structure:
/// - Ensures space after marker
/// - Handles mixed markers
fn fix_list_markers(content: &str) -> String {
    let lines: Vec<String> = content
        .lines()
        .map(|line| {
            let leading_spaces = line.len() - line.trim_start().len();
            let trimmed = line.trim_start();

            // Check for unordered list without space
            if trimmed.starts_with('-') && !trimmed.starts_with("---") && !trimmed.starts_with("- ")
            {
                let rest = &trimmed[1..];
                return format!("{}- {}", " ".repeat(leading_spaces), rest.trim_start());
            }

            if trimmed.starts_with('*') && !trimmed.starts_with("* ") {
                let rest = &trimmed[1..];
                return format!("{}* {}", " ".repeat(leading_spaces), rest.trim_start());
            }

            if trimmed.starts_with('+') && !trimmed.starts_with("+ ") {
                let rest = &trimmed[1..];
                return format!("{}+ {}", " ".repeat(leading_spaces), rest.trim_start());
            }

            // Check for ordered list without space
            if let Some(pos) = trimmed.find('.') {
                let before_dot = &trimmed[..pos];
                if before_dot.chars().all(|c| c.is_ascii_digit()) {
                    let after_dot = &trimmed[pos + 1..];
                    if !after_dot.starts_with(' ') && !after_dot.is_empty() {
                        return format!(
                            "{}{}. {}",
                            " ".repeat(leading_spaces),
                            before_dot,
                            after_dot.trim_start()
                        );
                    }
                }
            }

            line.to_string()
        })
        .collect();

    lines.join("\n")
}

/// Fix table pipe issues and collect diagnostics.
///
/// Fixes:
/// - Missing opening pipes: `Name|Age` → `|Name|Age|`
/// - Missing closing pipes: `Name|Age` → `|Name|Age|`
fn fix_table_pipes(content: &str, diagnostics: &mut Diagnostics) -> String {
    let mut lines: Vec<String> = Vec::new();
    let mut in_code_block = false;
    let mut line_number = 0;
    let mut in_table = false;
    let mut expected_columns: Option<usize> = None;

    for line in content.lines() {
        line_number += 1;
        let trimmed = line.trim();

        // Track code blocks
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_code_block = !in_code_block;
            lines.push(line.to_string());
            continue;
        }

        // Skip if in code block
        if in_code_block {
            lines.push(line.to_string());
            continue;
        }

        // Check if line contains pipes (potential table row)
        if trimmed.contains('|') && !trimmed.starts_with('>') {
            let mut fixed = trimmed.to_string();
            let mut had_issues = false;

            // Mark start of table
            if !in_table {
                in_table = true;
                expected_columns = None;
            }

            // Add opening pipe if missing
            if !fixed.starts_with('|') {
                fixed = format!("|{fixed}");
                had_issues = true;
            }

            // Add closing pipe if missing
            if !fixed.ends_with('|') {
                fixed.push('|');
                had_issues = true;
            }

            // Count columns
            let columns = fixed.split('|').filter(|s| !s.trim().is_empty()).count();

            // Check if this is a separator row
            let is_separator = fixed
                .split('|')
                .filter(|s| !s.trim().is_empty())
                .all(|cell| {
                    let trimmed = cell.trim();
                    trimmed.chars().all(|c| c == '-' || c == ':')
                });

            if !is_separator {
                // Set expected columns from first data row
                if expected_columns.is_none() {
                    expected_columns = Some(columns);
                } else if let Some(expected) = expected_columns {
                    // Check for column mismatch
                    if columns != expected {
                        diagnostics.add(
                            Diagnostic::new(
                                Severity::Warning,
                                DiagnosticKind::MalformedTable,
                                line_number,
                                format!(
                                    "Table has inconsistent columns: expected {expected}, found {columns}"
                                ),
                            )
                            .with_snippet(trimmed),
                        );
                    }
                }
            }

            if had_issues {
                diagnostics.add(
                    Diagnostic::new(
                        Severity::Info,
                        DiagnosticKind::MalformedTable,
                        line_number,
                        "Fixed missing table pipes",
                    )
                    .with_snippet(format!("{trimmed} → {fixed}")),
                );
            }

            lines.push(fixed);
        } else {
            // Not a table line
            if in_table {
                in_table = false;
                expected_columns = None;
            }
            lines.push(line.to_string());
        }
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_headings_no_space() {
        assert_eq!(fix_headings("#NoSpace"), "# NoSpace");
        assert_eq!(fix_headings("##Another"), "## Another");
    }

    #[test]
    fn test_fix_headings_trailing_hashes() {
        assert_eq!(fix_headings("####Trailing####"), "#### Trailing");
        assert_eq!(fix_headings("# Title #"), "# Title");
    }

    #[test]
    fn test_fix_headings_too_many_spaces() {
        assert_eq!(fix_headings("###  TooMany"), "### TooMany");
    }

    #[test]
    fn test_fix_headings_preserve_valid() {
        assert_eq!(fix_headings("# Valid Heading"), "# Valid Heading");
        assert_eq!(fix_headings("## Another Valid"), "## Another Valid");
    }

    #[test]
    fn test_fix_list_markers_no_space() {
        assert_eq!(fix_list_markers("-Item"), "- Item");
        assert_eq!(fix_list_markers("*Item"), "* Item");
        assert_eq!(fix_list_markers("+Item"), "+ Item");
    }

    #[test]
    fn test_fix_list_markers_ordered() {
        assert_eq!(fix_list_markers("1.Item"), "1. Item");
        assert_eq!(fix_list_markers("42.Something"), "42. Something");
    }

    #[test]
    fn test_fix_table_pipes() {
        let mut diagnostics = Diagnostics::new();
        assert_eq!(fix_table_pipes("Name|Age", &mut diagnostics), "|Name|Age|");
        assert_eq!(fix_table_pipes("|Name|Age", &mut diagnostics), "|Name|Age|");
        assert_eq!(fix_table_pipes("Name|Age|", &mut diagnostics), "|Name|Age|");
    }

    #[test]
    fn test_preprocess_combined() {
        let input = "#NoSpace\n-Item\nName|Age";
        let expected = "# NoSpace\n- Item\n|Name|Age|";
        let (result, _diagnostics) = preprocess(input);
        assert_eq!(result, expected);
    }
}
