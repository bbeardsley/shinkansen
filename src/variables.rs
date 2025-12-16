use std::collections::HashMap;
use std::path::PathBuf;

use crate::cli::Cli;
use crate::config::ConfigLoaderFactory;
use crate::error::{ContextExt, Result};

/// Collect all template variables with proper precedence
///
/// This function loads variables from multiple sources in order of precedence:
/// 1. Environment variables (lowest precedence) - only if specified via --env flag
/// 2. Config file variables - if a config file is specified via -c/--config flag
/// 3. Command-line variables (highest precedence) - specified via -D flag
///
/// Later sources override earlier ones for variables with the same name.
pub fn collect_variables(cli: &Cli) -> Result<HashMap<String, tera::Value>> {
    let mut variables = HashMap::new();

    // 1. Load environment variables (lowest precedence) - only if specified
    if cli.env.is_some() {
        collect_env_variables(&mut variables, cli)?;
    }

    // 2. Load config file variables
    if let Some(config_path) = &cli.config {
        collect_config_variables(&mut variables, config_path)?;
    }

    // 3. Load command-line variables (highest precedence)
    collect_cli_variables(&mut variables, &cli.variables)?;

    Ok(variables)
}

/// Collect variables from environment variables
///
/// Only loads variables that are explicitly listed in the --env flag
/// Variables that don't exist in the environment are silently ignored
pub fn collect_env_variables(
    variables: &mut HashMap<String, tera::Value>,
    cli: &Cli,
) -> Result<()> {
    load_env_variables(variables, cli)
}

/// Collect variables from a configuration file
///
/// Supports JSON, YAML, YML, and TOML file formats
/// Returns an error if the file format is unsupported or the file cannot be read
pub fn collect_config_variables(
    variables: &mut HashMap<String, tera::Value>,
    config_path: &PathBuf,
) -> Result<()> {
    load_config_file(variables, config_path)
}

/// Collect variables from command-line arguments
///
/// Variables are specified in KEY=VALUE format via the -D flag
/// Supports escaping special characters: \\ (backslash), \, (comma), \= (equals), \[ \], \{ \}
/// Multiple variables can be specified in one flag separated by commas: -D "a=1,b=2"
/// Returns an error if any variable is not in the correct format
pub fn collect_cli_variables(
    variables: &mut HashMap<String, tera::Value>,
    cli_vars: &[String],
) -> Result<()> {
    load_cli_variables(variables, cli_vars)
}

fn load_env_variables(variables: &mut HashMap<String, tera::Value>, cli: &Cli) -> Result<()> {
    if let Some(env_vars) = &cli.env {
        let var_names: Vec<&str> = env_vars.split(',').map(|s| s.trim()).collect();

        for var_name in var_names {
            if let Ok(value) = std::env::var(var_name) {
                // Unescape the value first, then convert to appropriate type
                let unescaped_value = unescape_value(&value);
                let tera_value = string_to_tera_value(&unescaped_value);
                variables.insert(var_name.to_string(), tera_value);
            }
        }
    }

    Ok(())
}

fn load_config_file(
    variables: &mut HashMap<String, tera::Value>,
    config_path: &PathBuf,
) -> Result<()> {
    let content = std::fs::read_to_string(config_path)
        .with_context(|| format!("Failed to read config file: {:?}", config_path))?;

    let ext = config_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    let loader = ConfigLoaderFactory::create_loader(ext).ok_or_else(|| {
        crate::error::ShinkansenError::ConfigParseError(
            "Unsupported config file format. Use .json, .yaml, .yml, or .toml".to_string(),
        )
    })?;

    let config = loader.load_config(&content)?;

    // Convert serde_json::Value to tera::Value
    for (key, value) in config.variables {
        variables.insert(key, json_to_tera_value(value));
    }

    Ok(())
}

fn json_to_tera_value(value: serde_json::Value) -> tera::Value {
    match value {
        serde_json::Value::Null => tera::Value::Null,
        serde_json::Value::Bool(b) => tera::Value::Bool(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                tera::Value::Number(i.into())
            } else if let Some(f) = n.as_f64() {
                // Handle NaN and infinite values by converting to null
                if f.is_finite() {
                    if let Some(num) = serde_json::Number::from_f64(f) {
                        tera::Value::Number(num)
                    } else {
                        tera::Value::Null
                    }
                } else {
                    tera::Value::Null
                }
            } else {
                tera::Value::Null
            }
        }
        serde_json::Value::String(s) => tera::Value::String(s),
        serde_json::Value::Array(arr) => {
            tera::Value::Array(arr.into_iter().map(json_to_tera_value).collect())
        }
        serde_json::Value::Object(obj) => {
            let map: HashMap<String, tera::Value> = obj
                .into_iter()
                .map(|(k, v)| (k, json_to_tera_value(v)))
                .collect();
            tera::Value::Object(map.into_iter().collect())
        }
    }
}

fn load_cli_variables(
    variables: &mut HashMap<String, tera::Value>,
    cli_vars: &[String],
) -> Result<()> {
    for var in cli_vars {
        // Split the argument on unescaped commas to handle multiple variables in one -D flag
        let var_parts = split_unescaped(var, ',');

        for single_var in var_parts {
            // Find the first unescaped '=' character
            let mut key_end = 0;
            let chars = single_var.chars().peekable();
            let mut in_escape = false;

            for ch in chars {
                if in_escape {
                    // Skip the escaped character
                    in_escape = false;
                } else if ch == '\\' {
                    // Start escape sequence
                    in_escape = true;
                } else if ch == '=' {
                    // Found unescaped '=' - this is the separator
                    break;
                }
                key_end += ch.len_utf8();
            }

            if key_end == 0 || key_end >= single_var.len() {
                return Err(crate::error::ShinkansenError::VariableParseError(format!(
                    "Invalid variable format: '{}'. Use KEY=VALUE",
                    single_var
                )));
            }

            let key = &single_var[..key_end];
            let value_with_escapes = &single_var[key_end + 1..]; // Skip the '='

            // Unescape the value
            let value = unescape_value(value_with_escapes);

            // Convert to appropriate type (number, bool, or string)
            let tera_value = string_to_tera_value(&value);

            // Handle nested keys (e.g., "foo.bar" becomes {"foo": {"bar": value}})
            if key.contains('.') {
                insert_nested_variable(variables, key, tera_value);
            } else {
                variables.insert(key.to_string(), tera_value);
            }
        }
    }

    Ok(())
}

/// Insert a variable with a dotted key path into a nested structure
/// For example, "foo.bar.baz" with value 5 becomes {"foo": {"bar": {"baz": 5}}}
fn insert_nested_variable(
    variables: &mut HashMap<String, tera::Value>,
    key_path: &str,
    value: tera::Value,
) {
    let parts: Vec<&str> = key_path.split('.').collect();

    if parts.len() == 1 {
        // No nesting needed
        variables.insert(parts[0].to_string(), value);
        return;
    }

    // Check if the root key already exists
    if let Some(existing_value) = variables.get(&parts[0].to_string()) {
        match existing_value {
            tera::Value::Object(existing_obj) => {
                // Root exists and is an object - merge into it
                let mut new_obj = existing_obj.clone();
                merge_nested_value(&mut new_obj, &parts, 1, value);
                variables.insert(parts[0].to_string(), tera::Value::Object(new_obj));
            }
            _ => {
                // Root exists but isn't an object - replace it with a new nested structure
                let nested_value = build_nested_object(&parts, 0, value);
                variables.insert(parts[0].to_string(), nested_value);
            }
        }
    } else {
        // Root doesn't exist - create new nested structure
        let nested_value = build_nested_object(&parts, 0, value);
        variables.insert(parts[0].to_string(), nested_value);
    }
}

/// Recursively build a nested object structure
fn build_nested_object(parts: &[&str], index: usize, value: tera::Value) -> tera::Value {
    if index == parts.len() - 1 {
        // Base case: we've reached the final part, return the value
        value
    } else {
        // Create an object with the next part as key and recursively build the rest
        let next_part = parts[index + 1];

        let mut map = tera::Map::new();
        let nested_value = build_nested_object(parts, index + 1, value);
        map.insert(next_part.to_string(), nested_value);

        tera::Value::Object(map)
    }
}

/// Merge a value into an existing nested object structure
fn merge_nested_value(
    obj: &mut tera::Map<String, tera::Value>,
    parts: &[&str],
    index: usize,
    value: tera::Value,
) {
    let current_part = parts[index];

    if index == parts.len() - 1 {
        // Final part - insert the value
        obj.insert(current_part.to_string(), value);
    } else {
        // Intermediate part - navigate deeper

        if let Some(existing_value) = obj.get(current_part) {
            match existing_value {
                tera::Value::Object(existing_obj) => {
                    // Key exists and is an object - merge into it
                    let mut new_obj = existing_obj.clone();
                    merge_nested_value(&mut new_obj, parts, index + 1, value);
                    obj.insert(current_part.to_string(), tera::Value::Object(new_obj));
                }
                _ => {
                    // Key exists but isn't an object - this is an error, but we'll replace it
                    let nested_value = build_nested_object(parts, index, value);
                    obj.insert(current_part.to_string(), nested_value);
                }
            }
        } else {
            // Key doesn't exist - create new nested structure
            let nested_value = build_nested_object(parts, index, value);
            obj.insert(current_part.to_string(), nested_value);
        }
    }
}

/// Split a string on unescaped occurrences of a delimiter character
/// Handles JSON-like structures (arrays and objects) by not splitting on commas inside brackets/braces
fn split_unescaped(s: &str, delimiter: char) -> Vec<&str> {
    let mut result = Vec::new();
    let mut current_start = 0;
    let chars: Vec<char> = s.chars().collect();
    let mut in_escape = false;
    let mut in_brackets: i32 = 0; // Track nesting level for []
    let mut in_braces: i32 = 0; // Track nesting level for {}
    let mut pos = 0;

    while pos < chars.len() {
        let ch = chars[pos];

        if in_escape {
            // Skip the escaped character
            in_escape = false;
        } else if ch == '\\' {
            // Start escape sequence
            in_escape = true;
        } else if ch == '[' {
            in_brackets += 1;
        } else if ch == ']' {
            in_brackets = in_brackets.saturating_sub(1);
        } else if ch == '{' {
            in_braces += 1;
        } else if ch == '}' {
            in_braces = in_braces.saturating_sub(1);
        } else if ch == delimiter && in_brackets == 0 && in_braces == 0 {
            // Found unescaped delimiter outside of brackets/braces - split here
            result.push(&s[current_start..pos]);
            current_start = pos + ch.len_utf8();
        }

        pos += ch.len_utf8();
    }

    // Add the remaining part
    if current_start < s.len() {
        result.push(&s[current_start..]);
    } else if result.is_empty() {
        // Handle empty string case
        result.push(s);
    }

    result
}

/// Unescape special characters in variable values
/// Supports: \\ (backslash), \, (comma), \= (equals), \[ (left bracket), \] (right bracket), \{ (left brace), \} (right brace)
fn unescape_value(value: &str) -> String {
    let mut result = String::new();
    let mut chars = value.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            // Handle escape sequences
            if let Some(next_ch) = chars.next() {
                match next_ch {
                    '\\' => result.push('\\'), // Escaped backslash
                    ',' => result.push(','),   // Escaped comma
                    '=' => result.push('='),   // Escaped equals
                    '[' => result.push('['),   // Escaped left bracket
                    ']' => result.push(']'),   // Escaped right bracket
                    '{' => result.push('{'),   // Escaped left brace
                    '}' => result.push('}'),   // Escaped right brace
                    _ => {
                        // Unknown escape sequence - keep both characters
                        result.push('\\');
                        result.push(next_ch);
                    }
                }
            } else {
                // Trailing backslash - keep it
                result.push('\\');
            }
        } else {
            result.push(ch);
        }
    }

    result
}

/// Convert a string value to the appropriate tera::Value type
/// Attempts to parse as JSON first (for arrays/objects), then number, then boolean, falls back to string
fn string_to_tera_value(value: &str) -> tera::Value {
    // Try to parse as JSON first (for arrays and objects)
    if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(value) {
        return json_to_tera_value(json_value);
    }

    // Try to parse as integer first
    if let Ok(int_val) = value.parse::<i64>() {
        return tera::Value::Number(int_val.into());
    }

    // Try to parse as float
    if let Ok(float_val) = value.parse::<f64>() {
        // Only return as number if it's a finite value
        if float_val.is_finite()
            && let Some(num) = serde_json::Number::from_f64(float_val)
        {
            return tera::Value::Number(num);
        }
    }

    // Try to parse as boolean
    if let Ok(bool_val) = value.parse::<bool>() {
        return tera::Value::Bool(bool_val);
    }

    // Fall back to string
    tera::Value::String(value.to_string())
}
