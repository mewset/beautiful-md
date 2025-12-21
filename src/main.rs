//! beautiful-md binary entry point.

#![allow(clippy::multiple_crate_versions)]

use anyhow::{Context, Result};
use beautiful_md::{format_file, format_markdown, Config};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

mod cli;

use cli::{Cli, Commands};

fn main() -> Result<()> {
    let args = Cli::parse_args();

    // Load configuration
    let config = if let Some(config_path) = &args.config {
        Config::from_file(config_path)
            .with_context(|| format!("Failed to load config from {}", config_path.display()))?
    } else {
        Config::load_default()
    };

    // Handle subcommands
    if let Some(command) = args.command {
        return handle_subcommand(command, &config);
    }

    // Handle main file processing
    if args.files.is_empty() {
        anyhow::bail!("No input files specified. Use --help for usage information.");
    }

    // Expand glob patterns if needed
    let files = if args.glob {
        expand_glob_patterns(&args.files)?
    } else {
        args.files.clone()
    };

    if args.check {
        return check_files(&files, &config);
    }

    if args.in_place {
        format_files_in_place(&files, &config, args.verbose)?;
    } else if let Some(output_path) = &args.output {
        if files.len() > 1 {
            anyhow::bail!("Cannot specify --output with multiple input files");
        }
        format_to_file(&files[0], output_path, &config)?;
    } else {
        // Default: output to stdout
        format_to_stdout(&files, &config)?;
    }

    Ok(())
}

/// Handle subcommands.
fn handle_subcommand(command: Commands, config: &Config) -> Result<()> {
    match command {
        Commands::Format { files, in_place } => {
            if in_place {
                format_files_in_place(&files, config, false)?;
            } else {
                format_to_stdout(&files, config)?;
            }
        }
        Commands::Config { output } => {
            config
                .save(&output)
                .with_context(|| format!("Failed to write config to {}", output.display()))?;
            println!("Configuration written to {}", output.display());
        }
        Commands::Check { files } => {
            return check_files(&files, config);
        }
    }
    Ok(())
}

/// Format files in-place.
fn format_files_in_place(
    files: &[std::path::PathBuf],
    config: &Config,
    verbose: bool,
) -> Result<()> {
    for file in files {
        if verbose {
            println!("Formatting {}", file.display());
        }
        format_file(file, config)
            .with_context(|| format!("Failed to format {}", file.display()))?;
    }
    Ok(())
}

/// Format file and write to specific output path.
fn format_to_file(input: &Path, output: &Path, config: &Config) -> Result<()> {
    let content =
        fs::read_to_string(input).with_context(|| format!("Failed to read {}", input.display()))?;

    let formatted = format_markdown(&content, config)
        .with_context(|| format!("Failed to format {}", input.display()))?;

    fs::write(output, formatted)
        .with_context(|| format!("Failed to write to {}", output.display()))?;

    Ok(())
}

/// Format files and write to stdout.
fn format_to_stdout(files: &[std::path::PathBuf], config: &Config) -> Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for file in files {
        let content = fs::read_to_string(file)
            .with_context(|| format!("Failed to read {}", file.display()))?;

        let formatted = format_markdown(&content, config)
            .with_context(|| format!("Failed to format {}", file.display()))?;

        writeln!(handle, "{formatted}").context("Failed to write to stdout")?;
    }

    Ok(())
}

/// Check if files need formatting.
fn check_files(files: &[std::path::PathBuf], config: &Config) -> Result<()> {
    let mut needs_formatting = Vec::new();

    for file in files {
        let content = fs::read_to_string(file)
            .with_context(|| format!("Failed to read {}", file.display()))?;

        let formatted = format_markdown(&content, config)
            .with_context(|| format!("Failed to format {}", file.display()))?;

        if content != formatted {
            needs_formatting.push(file.clone());
        }
    }

    if needs_formatting.is_empty() {
        println!("All files are properly formatted ✓");
        Ok(())
    } else {
        eprintln!("The following files need formatting:");
        for file in &needs_formatting {
            eprintln!("  - {}", file.display());
        }
        anyhow::bail!("{} file(s) need formatting", needs_formatting.len());
    }
}

/// Expand glob patterns into file paths.
fn expand_glob_patterns(patterns: &[std::path::PathBuf]) -> Result<Vec<std::path::PathBuf>> {
    let mut files = Vec::new();

    for pattern in patterns {
        let pattern_str = pattern.to_str().context("Invalid UTF-8 in glob pattern")?;

        for entry in glob::glob(pattern_str)
            .with_context(|| format!("Invalid glob pattern: {pattern_str}"))?
        {
            let path = entry.with_context(|| "Failed to read glob entry".to_string())?;
            if path.is_file() {
                files.push(path);
            }
        }
    }

    if files.is_empty() {
        anyhow::bail!("No files matched the specified pattern(s)");
    }

    Ok(files)
}
