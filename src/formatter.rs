//! Main formatting orchestration module.
//!
//! This module coordinates the various formatters to produce beautifully
//! formatted markdown.

use crate::config::Config;
use crate::diagnostics::Diagnostics;
use crate::error::{Error, Result};
use crate::formatters;
use crate::preprocessor;

/// Format markdown content according to configuration.
///
/// This is the main entry point for formatting. It orchestrates all
/// individual formatters in the correct order.
///
/// # Errors
///
/// Returns an error if parsing or formatting fails.
pub fn format(content: &str, config: &Config) -> Result<(String, Diagnostics)> {
    // Extract code blocks FIRST to preserve them completely verbatim
    let (protected_content, code_blocks) = formatters::extract_code_blocks_early(content);

    // Pre-process to fix common issues and collect diagnostics (without code blocks)
    let (preprocessed, diagnostics) = preprocessor::preprocess(&protected_content);

    // Parse markdown (without code blocks)
    let events = parse_markdown(&preprocessed);

    // Apply formatters in order
    let formatted = apply_formatters(&events, config)?;

    // Restore code blocks with original content preserved
    let final_content = formatters::restore_code_blocks_early(&formatted, &code_blocks, config);

    Ok((final_content, diagnostics))
}

/// Parse markdown content into events.
fn parse_markdown(content: &str) -> Vec<pulldown_cmark::Event<'_>> {
    use pulldown_cmark::{Options, Parser};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(content, options);
    parser.collect()
}

/// Apply all formatters to the parsed markdown.
fn apply_formatters(events: &[pulldown_cmark::Event<'_>], config: &Config) -> Result<String> {
    use pulldown_cmark_to_cmark::cmark;

    // For now, we'll use the basic cmark formatter
    // Individual formatters will be implemented next
    let mut buf = String::new();
    cmark(events.iter(), &mut buf)
        .map_err(|e| Error::FormattingError(format!("Failed to format markdown: {e}")))?;

    // Apply post-processing formatters
    let formatted = formatters::apply_all(&buf, config);

    Ok(formatted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_formatting() {
        let input = "# Hello\n\nWorld";
        let config = Config::default();
        let result = format(input, &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_markdown() {
        let input = "# Heading\n\nParagraph";
        let events = parse_markdown(input);
        assert!(!events.is_empty());
    }
}
