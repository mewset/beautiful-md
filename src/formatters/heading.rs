//! Heading formatting module.
//!
//! Handles spacing and normalization of Markdown headings.

use crate::config::HeadingConfig;
use crate::error::Result;

/// Format headings in markdown content.
///
/// Ensures consistent spacing before/after headings and space after `#` symbols.
pub fn format_headings(content: &str, config: &HeadingConfig) -> Result<String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut result: Vec<String> = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        if is_heading(trimmed) {
            // Add blank lines before heading
            if i > 0 && !result.is_empty() {
                // Count existing blank lines
                let mut blank_count = 0;
                for j in (0..result.len()).rev() {
                    if result[j].trim().is_empty() {
                        blank_count += 1;
                    } else {
                        break;
                    }
                }

                // Adjust to desired count
                while blank_count < config.blank_lines_before {
                    result.push(String::new());
                    blank_count += 1;
                }
                while blank_count > config.blank_lines_before {
                    result.pop();
                    blank_count -= 1;
                }
            }

            // Normalize heading (ensure space after #)
            let normalized = if config.space_after_hash {
                normalize_heading(trimmed)
            } else {
                trimmed.to_string()
            };
            result.push(normalized);

            // Add blank lines after heading
            for _ in 0..config.blank_lines_after {
                result.push(String::new());
            }

            // Skip existing blank lines after heading
            i += 1;
            while i < lines.len() && lines[i].trim().is_empty() {
                i += 1;
            }
            continue;
        }

        result.push(line.to_string());
        i += 1;
    }

    Ok(result.join("\n"))
}

/// Check if a line is a heading.
fn is_heading(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('#') && !trimmed.starts_with("```")
}

/// Normalize heading to ensure space after `#` symbols.
fn normalize_heading(line: &str) -> String {
    let trimmed = line.trim();
    let mut chars = trimmed.chars();
    let mut hashes = String::new();

    // Collect leading `#` characters
    for ch in chars.by_ref() {
        if ch == '#' {
            hashes.push(ch);
        } else {
            // Check if there's already a space
            if ch == ' ' {
                return format!("{hashes} {}", chars.as_str());
            }
            // Add space if missing
            return format!("{hashes} {ch}{}", chars.as_str());
        }
    }

    // Edge case: only hashes
    hashes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_heading() {
        assert_eq!(normalize_heading("#Heading"), "# Heading");
        assert_eq!(normalize_heading("# Heading"), "# Heading");
        assert_eq!(normalize_heading("##No Space"), "## No Space");
    }

    #[test]
    fn test_is_heading() {
        assert!(is_heading("# Heading"));
        assert!(is_heading("## Another"));
        assert!(!is_heading("Not a heading"));
        assert!(!is_heading("```code"));
    }

    #[test]
    fn test_format_headings() {
        let input = "# Heading\nText\n## Another";
        let config = HeadingConfig {
            blank_lines_before: 1,
            blank_lines_after: 1,
            space_after_hash: true,
        };

        let result = format_headings(input, &config).unwrap();
        assert!(result.contains("# Heading"));
    }
}
