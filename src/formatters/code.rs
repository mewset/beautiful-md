//! Code block formatting module.
//!
//! Handles formatting of fenced code blocks.

use crate::config::CodeConfig;
use crate::error::Result;

/// Format code blocks in markdown content.
///
/// Ensures consistent fence style and optionally adds language tags.
pub fn format_code_blocks(content: &str, config: &CodeConfig) -> Result<String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut in_code_block = false;
    let mut code_block_lines = Vec::new();
    let mut current_fence = String::new();

    for line in lines {
        let trimmed = line.trim();

        // Detect code fence
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            if !in_code_block {
                // Start of code block
                in_code_block = true;
                current_fence = if trimmed.starts_with("```") {
                    "```"
                } else {
                    "~~~"
                }
                .to_string();

                // Extract language tag if present
                let lang = if trimmed.len() > 3 {
                    trimmed[3..].trim().to_string()
                } else {
                    String::new()
                };

                code_block_lines.clear();
                code_block_lines.push((current_fence.clone(), lang));
            } else {
                // End of code block
                let (fence, lang) = &code_block_lines[0];
                result.push(format!("{fence}{lang}"));

                // Add code content
                for i in 1..code_block_lines.len() {
                    result.push(code_block_lines[i].1.clone());
                }

                // Add closing fence
                result.push(config.fence_style.clone());

                code_block_lines.clear();
                in_code_block = false;
                current_fence.clear();
                continue;
            }
        } else if in_code_block {
            code_block_lines.push((String::new(), line.to_string()));
        } else {
            result.push(line.to_string());
        }
    }

    // Handle unclosed code block
    if in_code_block {
        result.extend(code_block_lines.iter().map(|(_, line)| line.clone()));
    }

    Ok(result.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_code_blocks() {
        let input = "```rust\nfn main() {}\n```";
        let config = CodeConfig::default();
        let result = format_code_blocks(input, &config).unwrap();
        assert!(result.contains("```rust"));
        assert!(result.contains("fn main()"));
    }

    #[test]
    fn test_format_code_blocks_tilde() {
        let input = "~~~python\nprint('hello')\n~~~";
        let config = CodeConfig::default();
        let result = format_code_blocks(input, &config).unwrap();
        assert!(result.contains("python"));
    }
}
