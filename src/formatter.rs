//! Main formatting orchestration module.
//!
//! This module coordinates the various formatters to produce beautifully
//! formatted markdown.

use crate::config::Config;
use crate::error::{Error, Result};
use crate::formatters;

/// Format markdown content according to configuration.
///
/// This is the main entry point for formatting. It orchestrates all
/// individual formatters in the correct order.
///
/// # Errors
///
/// Returns an error if parsing or formatting fails.
pub fn format(content: &str, config: &Config) -> Result<String> {
    // Parse markdown
    let events = parse_markdown(content);

    // Apply formatters in order
    let formatted = apply_formatters(&events, config)?;

    Ok(formatted)
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
