# Shinkansen - CLI File Preprocessor

A fast, cross-platform command-line file preprocessor built with Rust that uses
[MiniJinja](https://github.com/mitsuhiko/minijinja) templates to transform files with dynamic
variables.

## Features

- **Flexible Input**: Process single files, multiple files, directories (with
  optional recursion), or stdin
- **Smart Output**: Write to stdout, single files, or directories while
  preserving structure
- **Multiple Config Formats**: Load variables from JSON, YAML, or TOML
  configuration files
- **Variable Precedence**: Layer variables from environment (explicit), config
  files, and CLI arguments
- **Environment Control**: Load specific environment variables with `--env` flag
- **Cross-Platform**: Builds on Linux, macOS, and (future) Windows
- **Powered by MiniJinja**: Full access to MiniJinja's powerful template syntax and
  filters

## Installation

### From Source

```bash
git clone https://github.com/bbeardsley/shinkansen
cd shinkansen
cargo build --release
# Binary will be at ./target/release/shinkansen
```

### Using Just

```bash
just install
```

This installs the binary to your Cargo bin directory (typically `~/.cargo/bin`).

### Usage

Add the binary to your PATH or use it directly:

```bash
./target/release/shinkansen --help
```

## Usage Examples

### Basic Usage

Process a single template file with CLI variables:

```bash
shinkansen template.txt -D name="John" -D app="MyApp" -o -
```

### Using Configuration Files

**JSON Config (config.json):**

```json
{
  "name": "Alice",
  "version": "1.0.0",
  "environment": "production"
}
```

```bash
shinkansen template.txt -c config.json -o output.txt
```

**YAML Config (config.yaml):**

```yaml
name: Bob
version: "2.0.0"
debug: true
```

```bash
shinkansen template.txt -c config.yaml -o -
```

**TOML Config (config.toml):**

```toml
name = "Charlie"
version = "3.0.0"
features = ["auth", "api"]
```

```bash
shinkansen template.txt -c config.toml -o output.txt
```

### Processing from Stdin

```bash
echo "Hello, {{ name }}!" | shinkansen - -D name="World"
# Output: Hello, World!

# Explicit stdout with -o -
echo "Hello, {{ name }}!" | shinkansen - -D name="World" -o -
```

### Variable Precedence

Variables are layered with increasing precedence:

1. Environment variables (only when specified with `--env`, lowest)
2. Config file variables
3. CLI arguments (highest)

```bash
export GREETING="Hi"
shinkansen template.txt -c config.yaml --env="GREETING" -D GREETING="Hello" -o -
# CLI value "Hello" wins
```

### Environment Variable Control

**Load specific environment variables:**

```bash
shinkansen template.txt --env="PATH,HOME,USER" -o -
```

### Escaping Special Characters

When using `-D` flag or environment variables, you can escape special
characters:

- `\\` - escaped backslash
- `\,` - escaped comma
- `\=` - escaped equals
- `\[` - escaped left bracket
- `\]` - escaped right bracket
- `\{` - escaped left curly brace
- `\}` - escaped right curly brace

**Examples:**

```bash
# Escape curly brackets in command line
shinkansen template.txt -D "message=Hello\{world\}"

# Escape multiple characters
shinkansen template.txt -D "data=key\=value\,more\\data"

# Environment variables with escaping
export VAR="test\{curly\}brackets"
shinkansen template.txt --env VAR
```

Environment variables are **not** loaded automatically. You must explicitly
specify which variables to load using the `--env` flag. Multiple variables can
be specified as a comma-separated list.

### Directory Processing

**Process all files in a directory (non-recursive):**

```bash
shinkansen templates/ -o output/ -D env="prod"
```

**Process directory recursively:**

```bash
shinkansen templates/ -r -o output/ -c config.json
```

Directory structure is preserved in the output:

```text
templates/
  ├── app.conf
  └── services/
      └── api.conf

→ output/
  ├── app.conf
  └── services/
      └── api.conf
```

### Multiple Files

```bash
shinkansen file1.txt file2.txt file3.txt -o output_dir/ -D version="1.0"
```

### Output Options

**To stdout (default for single input):**

```bash
shinkansen template.txt -D var="value"
# Or explicitly with -o -
shinkansen template.txt -D var="value" -o -
```

**To a specific file:**

```bash
shinkansen template.txt -D var="value" -o result.txt
```

**To a directory:**

```bash
shinkansen template.txt -D var="value" -o output/
# Creates output/template.txt
```

## Template Syntax

Shinkansen uses MiniJinja templates. Here are some common patterns:

### Variables

```tera
Hello, {{ name }}!
Version: {{ version }}
```

### Filters

```tera
{{ name | upper }}
{{ email | default(value="no-email") }}
{{ price | round(precision=2) }}
```

### Conditionals

```tera
{% if debug %}
Debug mode is ON
{% else %}
Production mode
{% endif %}
```

### Loops

```tera
{% for item in items %}
- {{ item }}
{% endfor %}
```

### Comments

```tera
{# This is a comment and won't appear in output #}
```

For complete MiniJinja syntax documentation, see:
<https://docs.rs/minijinja/latest/minijinja/>

## Input/Output Rules

| Input Type     | Valid Output Options       |
| -------------- | -------------------------- |
| Single file    | File, directory, or stdout |
| Multiple files | Directory only             |
| Directory      | Directory only             |
| Stdin          | File, directory, or stdout |

## Common Use Cases

### Configuration Management

Generate configuration files for different environments:

```bash
# Development
shinkansen app.conf.template -c dev.yaml -o config/dev/app.conf

# Production
shinkansen app.conf.template -c prod.yaml -o config/prod/app.conf
```

### Code Generation

```bash
shinkansen api_template.rs -c schema.json -o src/generated/api.rs
```

### Documentation

```bash
shinkansen README.template.md -D version="1.2.3" -D date="2024-01-15" -o README.md
```

### Batch Processing

```bash
shinkansen templates/ -r -c project.yaml -o output/
```

## Building for Different Platforms

The project is designed to be cross-platform.

**Linux:**

```bash
cargo build --release
```

**macOS:**

```bash
cargo build --release
```

**Windows:**

```bash
cargo build --release
```

Cross-compilation:

```bash
# For Windows from Linux/macOS
cargo build --release --target x86_64-pc-windows-msvc
```

## Error Handling

Shinkansen provides helpful error messages for common issues:

- Missing required template variables
- Invalid template syntax
- File/directory access errors
- Invalid input/output combinations
- Config file parsing errors

## Requirements

- Rust 2024 edition or later
- Cargo
- Optional: [just](https://github.com/casey/just) for build tasks

## Dependencies

- [clap](https://github.com/clap-rs/clap) - Command-line argument parsing with
  derive macros
- [clap_complete](https://crates.io/crates/clap_complete) - Shell completion
  generation
- [minijinja](https://github.com/mitsuhiko/minijinja) - Template engine
- [serde](https://github.com/serde-rs/serde) - Serialization framework
- [serde_json](https://github.com/serde-rs/json) - JSON support
- [serde_yaml](https://github.com/dtolnay/serde-yaml) - YAML support
- [toml](https://github.com/toml-rs/toml) - TOML support
- [walkdir](https://github.com/BurntSushi/walkdir) - Directory traversal
- [tempfile](https://github.com/Stebalien/tempfile) - Temporary file handling
  (dev and runtime)

## Examples

See the `examples/` directory for complete, working examples:

- **project.md.template** - Project README template with conditionals and loops
- **nginx.conf.template** - Nginx configuration template
- **config.json** - Sample JSON configuration
- **EXAMPLES.md** - Detailed usage examples

Run examples directly:

```bash
# Simple template with CLI variables
./target/release/shinkansen examples/project.md.template \
  -D project_name="My Project" \
  -D version="2.0.0" \
  -o -

# Using config file
./target/release/shinkansen examples/project.md.template \
  -c examples/config.json \
  -o -
```

## License

MIT License - see repository for details.

## Troubleshooting

### Variable Not Found Error

If you see `Variable 'x' not found in context`, ensure:

1. The variable is defined via `-D`, config file, or environment
2. The variable name matches exactly (case-sensitive)
3. Use the `default` filter for optional variables:
   `{{ var | default(value="fallback") }}`

### Multiple Inputs Require Directory

When processing multiple files, specify an output directory:

```bash
shinkansen file1.txt file2.txt -o output_dir/
```
