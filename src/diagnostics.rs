//! Diagnostic and warning system for markdown formatting issues.
//!
//! Collects warnings about problematic markdown that couldn't be automatically fixed.

use owo_colors::{OwoColorize, Stream, Style};
use std::fmt;

/// Severity level of a diagnostic message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// Warning: Issue detected but formatting continued
    Warning,
    /// Info: Non-critical information
    Info,
}

/// Type of diagnostic issue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticKind {
    /// Table with structural issues (missing pipes, uneven columns)
    MalformedTable,
    /// Code block without proper closing fence
    UnclosedCodeBlock,
    /// Other markdown issues
    Other,
}

/// A diagnostic message about a formatting issue.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// Severity level
    pub severity: Severity,
    /// Type of issue
    pub kind: DiagnosticKind,
    /// Line number where issue was found (1-indexed)
    pub line: usize,
    /// Human-readable message
    pub message: String,
    /// Optional snippet of the problematic line
    pub snippet: Option<String>,
}

impl Diagnostic {
    /// Create a new diagnostic.
    pub fn new(
        severity: Severity,
        kind: DiagnosticKind,
        line: usize,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            kind,
            line,
            message: message.into(),
            snippet: None,
        }
    }

    /// Add a code snippet to the diagnostic.
    #[must_use]
    pub fn with_snippet(mut self, snippet: impl Into<String>) -> Self {
        self.snippet = Some(snippet.into());
        self
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let severity_icon = match self.severity {
            Severity::Warning => "⚠️",
            Severity::Info => "ℹ️",
        };

        write!(f, "{severity_icon} Line {}: {}", self.line, self.message)?;

        if let Some(snippet) = &self.snippet {
            write!(f, "\n  │ {snippet}")?;
        }

        Ok(())
    }
}

/// Collection of diagnostics.
#[derive(Debug, Default, Clone)]
pub struct Diagnostics {
    messages: Vec<Diagnostic>,
}

impl Diagnostics {
    /// Create a new empty diagnostics collection.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    /// Add a diagnostic message.
    pub fn add(&mut self, diagnostic: Diagnostic) {
        self.messages.push(diagnostic);
    }

    /// Add a warning.
    pub fn warn(&mut self, kind: DiagnosticKind, line: usize, message: impl Into<String>) {
        self.add(Diagnostic::new(Severity::Warning, kind, line, message));
    }

    /// Add an info message.
    pub fn info(&mut self, kind: DiagnosticKind, line: usize, message: impl Into<String>) {
        self.add(Diagnostic::new(Severity::Info, kind, line, message));
    }

    /// Check if there are any diagnostics.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Get the number of diagnostics.
    #[must_use]
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// Get all diagnostics.
    #[must_use]
    pub fn messages(&self) -> &[Diagnostic] {
        &self.messages
    }

    /// Get diagnostics by severity.
    #[must_use]
    pub fn by_severity(&self, severity: Severity) -> Vec<&Diagnostic> {
        self.messages
            .iter()
            .filter(|d| d.severity == severity)
            .collect()
    }

    /// Print diagnostics to stderr.
    pub fn print_to_stderr(&self) {
        self.print_to_stderr_impl(false);
    }

    /// Print diagnostics to stderr with colors.
    pub fn print_to_stderr_colored(&self) {
        self.print_to_stderr_impl(true);
    }

    /// Internal implementation for printing diagnostics.
    fn print_to_stderr_impl(&self, use_colors: bool) {
        if self.is_empty() {
            return;
        }

        let len = self.len();

        if use_colors {
            let header = format!("{len} issues found:");
            let header_style = Style::new().yellow().bold();
            eprintln!(
                "\n{}",
                header.if_supports_color(Stream::Stderr, |text| text.style(header_style))
            );
            for diagnostic in &self.messages {
                Self::print_diagnostic_colored(diagnostic);
            }
        } else {
            eprintln!("\n{len} issues found:");
            for diagnostic in &self.messages {
                eprintln!("{diagnostic}");
            }
        }
        eprintln!();
    }

    /// Print a single diagnostic with colors.
    fn print_diagnostic_colored(diagnostic: &Diagnostic) {
        let severity_icon = match diagnostic.severity {
            Severity::Warning => "⚠️",
            Severity::Info => "ℹ️",
        };

        let line_text = format!("Line {}", diagnostic.line);
        eprint!(
            "{} {}: ",
            severity_icon,
            line_text.if_supports_color(Stream::Stderr, |text| text.dimmed())
        );

        match diagnostic.severity {
            Severity::Warning => eprintln!(
                "{}",
                diagnostic
                    .message
                    .if_supports_color(Stream::Stderr, |text| text.yellow())
            ),
            Severity::Info => eprintln!(
                "{}",
                diagnostic
                    .message
                    .if_supports_color(Stream::Stderr, |text| text.cyan())
            ),
        }

        if let Some(snippet) = &diagnostic.snippet {
            eprintln!(
                "  {} {}",
                "│".if_supports_color(Stream::Stderr, |text| text.dimmed()),
                snippet.if_supports_color(Stream::Stderr, |text| text.dimmed())
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_creation() {
        let diag = Diagnostic::new(
            Severity::Warning,
            DiagnosticKind::MalformedTable,
            42,
            "Test message",
        );
        assert_eq!(diag.line, 42);
        assert_eq!(diag.message, "Test message");
    }

    #[test]
    fn test_diagnostics_collection() {
        let mut diags = Diagnostics::new();
        assert!(diags.is_empty());

        diags.warn(DiagnosticKind::MalformedTable, 10, "Test");
        assert_eq!(diags.len(), 1);

        let warnings = diags.by_severity(Severity::Warning);
        assert_eq!(warnings.len(), 1);
    }
}
