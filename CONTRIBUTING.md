# Contributing to beautiful-md

Thank you for your interest in contributing to beautiful-md! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Please be respectful and constructive in all interactions.

## How to Contribute

### Reporting Bugs

If you find a bug, please open an issue with:

- A clear, descriptive title
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Your environment (OS, Rust version, beautiful-md version)
- Sample markdown that demonstrates the issue

### Suggesting Features

Feature requests are welcome! Please open an issue describing:

- The problem you're trying to solve
- Your proposed solution
- Alternative solutions you've considered
- Example use cases

### Pull Requests

1. **Fork and Clone**

   ```bash
   git clone https://github.com/mewset/beautiful-md.git
   cd beautiful-md
   ```

2. **Create a Branch**

   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make Your Changes**

   - Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
   - Write tests for new functionality
   - Update documentation as needed
   - Ensure all tests pass: `cargo test`
   - Run clippy: `cargo clippy -- -D warnings`
   - Format code: `cargo fmt`

4. **Commit Your Changes**

   Write clear, descriptive commit messages:

   ```
   Add table column alignment feature

   - Implement column width calculation
   - Add padding configuration option
   - Add tests for table formatting
   ```

5. **Push and Create PR**

   ```bash
   git push origin feature/your-feature-name
   ```

   Then create a pull request on GitHub with:
   - Description of changes
   - Related issue numbers
   - Testing performed

## Development Guidelines

### Code Style

- Follow the official [Rust Style Guide](https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md)
- Use `cargo fmt` with the project's `rustfmt.toml`
- Pass all `cargo clippy` checks
- Maintain the existing code organization

### Testing

- Write unit tests for new functions
- Write integration tests for new features
- Ensure test coverage for edge cases
- Run `cargo test` before submitting

Example test structure:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_name() {
        let input = "...";
        let expected = "...";
        let result = format_feature(input);
        assert_eq!(result, expected);
    }
}
```

### Documentation

- Add rustdoc comments for public APIs
- Include examples in documentation
- Update README.md for user-facing changes
- Keep CHANGELOG.md updated

Example documentation:

```rust
/// Format tables in markdown content.
///
/// Aligns columns, adds padding, and ensures consistent spacing.
///
/// # Errors
///
/// Returns an error if the content cannot be parsed.
///
/// # Example
///
/// ```
/// use beautiful_md::format_tables;
///
/// let input = "|Name|Age|\n|---|---|\n|Alice|30|";
/// let result = format_tables(input);
/// ```
pub fn format_tables(content: &str) -> Result<String> {
    // ...
}
```

### Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `test:` Test changes
- `refactor:` Code refactoring
- `perf:` Performance improvements
- `chore:` Maintenance tasks

## Project Structure

```
beautiful-md/
├── src/
│   ├── lib.rs          # Library public API
│   ├── main.rs         # CLI binary
│   ├── cli.rs          # CLI argument parsing
│   ├── config.rs       # Configuration types
│   ├── error.rs        # Error types
│   ├── formatter.rs    # Main formatting logic
│   └── formatters/     # Individual formatters
│       ├── mod.rs
│       ├── table.rs
│       ├── heading.rs
│       ├── list.rs
│       └── code.rs
├── tests/              # Integration tests
├── examples/           # Example files
└── benches/            # Benchmarks
```

## Getting Help

- Open an issue for questions
- Join discussions in existing issues
- Refer to the [documentation](https://docs.rs/beautiful-md)

## License

By contributing, you agree that your contributions will be licensed under the same terms as the project (MIT OR Apache-2.0).
