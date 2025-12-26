//! Command-line interface for beautiful-md.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// A CLI tool to format and beautify Markdown files.
#[derive(Parser, Debug)]
#[command(name = "beautiful-md")]
#[command(author, version, about, long_about = None)]
#[allow(clippy::struct_excessive_bools)]
pub struct Cli {
    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Input file(s) or glob pattern
    #[arg(value_name = "FILE")]
    pub files: Vec<PathBuf>,

    /// Configuration file path
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Modify files in-place
    #[arg(short, long)]
    pub in_place: bool,

    /// Output file (only for single input file)
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Write to stdout (default when not --in-place)
    #[arg(long)]
    pub stdout: bool,

    /// Process files matching glob pattern
    #[arg(short, long)]
    pub glob: bool,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Check mode: exit with error if files need formatting
    #[arg(long)]
    pub check: bool,

    /// Dry run: analyze files and report issues without modifying them
    #[arg(long)]
    pub dry_run: bool,

    /// Disable colored output
    #[arg(long)]
    pub no_color: bool,
}

/// Subcommands for beautiful-md.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Format markdown file(s)
    Format {
        /// Files to format
        files: Vec<PathBuf>,

        /// Modify files in-place
        #[arg(short, long)]
        in_place: bool,
    },

    /// Generate a default configuration file
    Config {
        /// Output path for config file
        #[arg(default_value = ".beautiful-md.toml")]
        output: PathBuf,
    },

    /// Check if files need formatting
    Check {
        /// Files to check
        files: Vec<PathBuf>,
    },
}

impl Cli {
    /// Create a new CLI instance from command-line arguments.
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        let cli = Cli::parse_from(["beautiful-md", "test.md"]);
        assert_eq!(cli.files.len(), 1);
        assert!(!cli.in_place);
    }

    #[test]
    fn test_cli_in_place() {
        let cli = Cli::parse_from(["beautiful-md", "--in-place", "test.md"]);
        assert!(cli.in_place);
    }
}
