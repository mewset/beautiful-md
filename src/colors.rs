//! Terminal color utilities for beautiful-md CLI output.
//!
//! Provides consistent color styling with automatic TTY detection and `NO_COLOR` support.

use owo_colors::{OwoColorize, Stream};

/// Style for success messages (green).
pub fn success(text: impl AsRef<str>) -> String {
    format!(
        "{}",
        text.as_ref()
            .if_supports_color(Stream::Stdout, |t| t.green())
    )
}

/// Style for error messages (red).
pub fn error(text: impl AsRef<str>) -> String {
    format!(
        "{}",
        text.as_ref().if_supports_color(Stream::Stderr, |t| t.red())
    )
}

/// Style for warning messages (yellow).
pub fn warning(text: impl AsRef<str>) -> String {
    format!(
        "{}",
        text.as_ref()
            .if_supports_color(Stream::Stderr, |t| t.yellow())
    )
}

/// Style for info messages (cyan).
pub fn info(text: impl AsRef<str>) -> String {
    format!(
        "{}",
        text.as_ref()
            .if_supports_color(Stream::Stderr, |t| t.cyan())
    )
}

/// Style for file paths (bright cyan).
pub fn path(text: impl AsRef<str>) -> String {
    format!(
        "{}",
        text.as_ref()
            .if_supports_color(Stream::Stderr, |t| t.bright_cyan())
    )
}

/// Style for line numbers (dimmed).
#[allow(dead_code)]
pub fn line_number(text: impl AsRef<str>) -> String {
    format!(
        "{}",
        text.as_ref()
            .if_supports_color(Stream::Stderr, |t| t.dimmed())
    )
}

/// Style for emphasis (bold).
#[allow(dead_code)]
pub fn bold(text: impl AsRef<str>) -> String {
    format!(
        "{}",
        text.as_ref()
            .if_supports_color(Stream::Stdout, |t| t.bold())
    )
}

/// Style for code snippets (dimmed).
#[allow(dead_code)]
pub fn snippet(text: impl AsRef<str>) -> String {
    format!(
        "{}",
        text.as_ref()
            .if_supports_color(Stream::Stderr, |t| t.dimmed())
    )
}

/// Set global color mode override.
///
/// This is used to implement the --no-color flag.
pub fn set_override(enabled: bool) {
    if !enabled {
        owo_colors::set_override(false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_functions_dont_panic() {
        // Colors might be disabled in test environment, but functions should work
        let _ = success("test");
        let _ = error("test");
        let _ = warning("test");
        let _ = info("test");
        let _ = path("test");
        let _ = line_number("test");
        let _ = bold("test");
        let _ = snippet("test");
    }

    #[test]
    fn test_set_override() {
        set_override(false);
        // Should not panic and should disable colors globally
        let result = success("test");
        assert_eq!(result, "test");
    }
}
