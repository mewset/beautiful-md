# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [Unreleased]


## [0.3.3] - 2025-12-27


### Changed

- **Default heading spacing**: Changed `blank_lines_before` from 2 to 1 to follow standard Markdown conventions
  - Users preferring the previous style can configure `blank_lines_before = 2` in `.beautiful-md.toml`
  - Provides more compact, standards-compliant output by default

### Improved

- **Code quality improvements** in preprocessor module:
  - Added `MAX_HEADING_LEVEL` constant for better code clarity
  - Extracted `format_list_line()` helper to reduce code duplication
  - Added documentation for defensive code block checks
  - Applied inline format args for clippy compliance
- **Documentation**: Updated CONTRIBUTING.md with pre-commit CI check workflow (`make check`)


## [0.3.2] - 2025-12-27


### Fixed

- **Critical**: Formatter now idempotent - no longer adds extra blank lines on repeated formatting
  - Preprocessor correctly distinguishes bold text (`**text**`) from list markers (`*`)
  - Fixed bug where `**Table of Contents:**` was corrupted to `* *Table of Contents:**`
  - Prevents progressive corruption on large files with repeated formatting passes
- Added test coverage for bold text before lists edge case


## [0.3.1] - 2025-12-26


### Fixed

- **Critical**: Code block content now preserved completely verbatim
  - Code blocks are extracted before any markdown processing
  - Prevents formatters from modifying comments, indentation, or blank lines inside code
  - Fixes issue where `# comments` in bash/shell code were treated as markdown headings
  - Preserves exact spacing and indentation in all code blocks (Python, JavaScript, etc.)


## [0.3.0] - 2025-12-26


### Added

- **Colored Terminal Output**: Beautiful colored CLI output with automatic TTY detection
  - Success messages in green (✓)
  - Error messages in red (✗)
  - Warning messages in yellow (⚠️)
  - Info messages in cyan (ℹ️)
  - File paths in bright cyan
  - Dimmed line numbers and code snippets
- **`--no-color` Flag**: Explicitly disable colored output
- **Smart Color Detection**:
  - Automatically disabled when output is piped or redirected
  - Respects `NO_COLOR` environment variable
  - Auto-detects CI/CD environments
  - Perfect for GitHub Actions, GitLab CI, etc.


### Changed

- Added `owo-colors` dependency with `supports-colors` feature
- New `colors.rs` module for consistent color styling
- Updated diagnostics system with `print_to_stderr_colored()` method


## [0.2.0] - 2024-12-21


### Added

- **Intelligent Preprocessor**: Automatically fixes common markdown issues before formatting:
  - Headings without spaces: `#NoSpace` → `# NoSpace`
  - Trailing hashes: `####Title####` → `#### Title`
  - List markers without spaces: `-Item` → `- Item`
    \|- Missing table pipes: `Name|Age` → `|Name|Age|`\|
- **Diagnostics System**: Comprehensive issue reporting with line numbers
  - Warning severity for serious issues (e.g., inconsistent table columns)
  - Info severity for auto-fixed issues (e.g., missing pipes)
  - Detailed output with code snippets showing before/after
- **Dry-run Mode** (`--dry-run`): Analyze files and report issues without modifying them
  - Shows all detected problems with line numbers
  - Displays summary of total issues found
  - Helps users understand what will be changed before running formatter


### Changed

- All formatting functions now return diagnostics alongside formatted content
- Error messages now print to stderr for better CLI composability


## [0.1.1] - 2024-12-21


### Added

- docs.rs metadata configuration for proper documentation builds
- Ko-fi sponsor link
- Automated GitHub releases workflow for pre-compiled binaries


### Fixed

- Documentation badge on docs.rs


## [0.1.0] - 2024-12-21


### Added

- Initial release
- Table formatting with column alignment and padding
- Heading spacing normalization
- List indentation and marker consistency
- Code block formatting with fence style normalization
- CLI with multiple output modes:
  - In-place editing (`--in-place`)
  - Output to file (`--output`)
  - Output to stdout (default)
  - Check mode (`--check`)
- TOML configuration support (`.beautiful-md.toml`)
- Configurable formatting rules:
  - Table alignment, padding, column width
  - Heading blank lines and spacing
  - List indentation and markers
  - Code block fence styles
- Comprehensive test suite
- CI/CD pipeline with GitHub Actions
- Full documentation (README, CONTRIBUTING, rustdoc)
- Dual licensing (MIT OR Apache-2.0)

[Unreleased]: https://github.com/mewset/beautiful-md/compare/v0.3.2...HEAD
[0.3.2]: https://github.com/mewset/beautiful-md/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/mewset/beautiful-md/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/mewset/beautiful-md/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/mewset/beautiful-md/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/mewset/beautiful-md/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/mewset/beautiful-md/releases/tag/v0.1.0
