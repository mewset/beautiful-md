# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[unreleased]: https://github.com/mewset/beautiful-md/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/mewset/beautiful-md/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/mewset/beautiful-md/releases/tag/v0.1.0
