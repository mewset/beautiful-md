# beautiful-md

[![CI](https://github.com/mewset/beautiful-md/workflows/CI/badge.svg)](https://github.com/mewset/beautiful-md/actions)
[![Crates.io](https://img.shields.io/crates/v/beautiful-md.svg)](https://crates.io/crates/beautiful-md)
[![Documentation](https://docs.rs/beautiful-md/badge.svg)](https://docs.rs/beautiful-md)
[![License](https://img.shields.io/crates/l/beautiful-md.svg)](LICENSE-MIT)

A CLI tool and Rust library to format and beautify Markdown files with configurable style rules.


## Features

- ✨ **Table Formatting**: Align columns, consistent padding, and clean appearance
- 📝 **Heading Normalization**: Consistent spacing and hash mark formatting
- 📋 **List Formatting**: Uniform indentation and bullet markers
- 💻 **Code Block Styling**: Consistent fence styles and language tags
- ⚙️ **Configurable**: Customize formatting rules via TOML configuration
- 🚀 **Fast**: Written in Rust for optimal performance
- 📦 **Multiple Modes**: In-place editing, stdout output, or file output


## Installation


### From crates.io

```bash
cargo install beautiful-md
```


### From source

```bash
git clone https://github.com/mewset/beautiful-md.git
cd beautiful-md
cargo install --path .
```


## Usage


### Command Line

```bash
# Format and output to stdout
beautiful-md README.md

# Format file in-place
beautiful-md --in-place README.md

# Format multiple files
beautiful-md --in-place *.md

# Format with custom config
beautiful-md --config my-config.toml README.md

# Check if files need formatting (useful for CI)
beautiful-md --check README.md

# Generate default configuration file
beautiful-md config
```


### As a Library

```rust
use beautiful_md::{Config, format_markdown};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let markdown = "# Heading\n\n|Name|Age|\n|---|---|\n|Alice|30|";
    let config = Config::default();
    let formatted = format_markdown(markdown, &config)?;
    println!("{}", formatted);
    Ok(())
}
```


## Configuration

Create a `.beautiful-md.toml` file in your project root or home directory:

```toml
[tables]
align = true
min_column_width = 3
padding = 1

[headings]
blank_lines_before = 1
blank_lines_after = 1
space_after_hash = true

[lists]
indent_size = 2
marker = "-"
normalize_numbers = true

[code]
ensure_language_tag = false
fence_style = "```"
```


### Configuration Options


#### Tables

- `align` (bool): Enable column alignment
- `min_column_width` (usize): Minimum width for columns
- `padding` (usize): Spaces around cell content


#### Headings

- `blank_lines_before` (usize): Empty lines before headings
- `blank_lines_after` (usize): Empty lines after headings
- `space_after_hash` (bool): Ensure space after `#` symbols


#### Lists

- `indent_size` (usize): Spaces per indentation level
- `marker` (string): Bullet character (`-`, `*`, or `+`)
- `normalize_numbers` (bool): Fix ordered list numbering


#### Code

- `ensure_language_tag` (bool): Require language tags
- `fence_style` (string): Fence style (```` ``` ```` or `~~~`)


## Examples


### Before

```markdown
#Heading Without Space

|Name|Age|City|
|---|---|---|
|Alice|30|Stockholm|
|Bob|25|Göteborg|

- Item 1
* Item 2
+ Item 3
```


### After

```markdown
# Heading With Space

| Name  | Age | City      |
| ----- | --- | --------- |
| Alice | 30  | Stockholm |
| Bob   | 25  | Göteborg  |

- Item 1
- Item 2
- Item 3
```


## Development


### Prerequisites

- Rust 1.70 or later
- Cargo


### Building

```bash
cargo build --release
```


### Testing

```bash
cargo test
```


### Linting

```bash
cargo clippy -- -D warnings
cargo fmt --check
```


## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.


## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.