# Shinkansen Examples

This directory contains example templates and configurations to demonstrate Shinkansen's capabilities.

## Quick Start

### Example 1: Simple Template with CLI Variables

```bash
./target/release/shinkansen examples/project.md.template \
  -D project_name="My Project" \
  -D version="2.0.0" \
  -D environment="development" \
  -o -
```

### Example 2: Using JSON Configuration

```bash
./target/release/shinkansen examples/project.md.template \
  -c examples/config.json \
  -o -
```

### Example 3: Nginx Configuration with Multiple Sources

```bash
# Using environment variables + CLI overrides
export BACKEND_HOST=localhost
export BACKEND_PORT=8000

./target/release/shinkansen examples/nginx.conf.template \
  --env="BACKEND_HOST,BACKEND_PORT" \
  -D domain="myapp.com" \
  -D port="443" \
  -D ssl_enabled="true" \
  -D ssl_cert="/etc/ssl/cert.pem" \
  -D ssl_key="/etc/ssl/key.pem" \
  -o -
```

### Example 4: Variable Precedence

Demonstrates how CLI variables override config file variables:

```bash
./target/release/shinkansen examples/project.md.template \
  -c examples/config.json \
  -D environment="staging" \
  -D debug="true" \
  -o -
```

The output will show:
- `environment` as "STAGING" (CLI override)
- `debug` as "true" (CLI override)
- Other variables from config.json

### Example 5: Stdin Processing

```bash
echo "Hello, {{ name }}! Today is {{ day }}." | \
  ./target/release/shinkansen - \
  -D name="World" \
  -D day="Monday"
```

## Files

- `project.md.template` - A project README template with conditionals and loops
- `nginx.conf.template` - An nginx configuration template
- `config.json` - Sample JSON configuration file
- `EXAMPLES.md` - This file
