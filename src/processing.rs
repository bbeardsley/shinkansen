use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::cli::Cli;
use crate::error::{ContextExt, Result};
use crate::output::{determine_output_destination, write_to_output};
use crate::platform::{get_max_path_length, normalize_path};
use std::path::Component;

/// Validate a path for security issues
fn validate_path(path: &Path) -> Result<()> {
    // Prevent path traversal attacks
    if path
        .components()
        .any(|comp| matches!(comp, Component::ParentDir))
    {
        return Err(crate::error::ShinkansenError::SecurityError(
            "Invalid path - contains parent directory references".to_string(),
        ));
    }

    // Check path length and validity using platform-specific limit
    if path.as_os_str().len() > get_max_path_length() {
        return Err(crate::error::ShinkansenError::ValidationError(format!(
            "Path too long (max {} characters)",
            get_max_path_length()
        )));
    }

    // Check for invalid characters
    if path
        .as_os_str()
        .to_string_lossy()
        .contains(|c: char| c.is_control())
    {
        return Err(crate::error::ShinkansenError::SecurityError(
            "Path contains invalid control characters".to_string(),
        ));
    }

    Ok(())
}

/// Validate input/output combinations
pub fn validate_args(cli: &Cli) -> Result<()> {
    // Check if stdin is being used (either explicitly with "-" or implicitly with no inputs)
    let using_stdin = cli.inputs.is_empty() || (cli.inputs.len() == 1 && cli.inputs[0] == "-");

    // Check if multiple inputs include stdin
    if cli.inputs.len() > 1 && cli.inputs.iter().any(|i| i == "-") {
        return Err(crate::error::ShinkansenError::ValidationError(
            "Cannot use '-' (stdin) with other input files".to_string(),
        ));
    }

    // Check if stdin is used with directory output (which is invalid)
    if using_stdin
        && cli.output.as_ref().is_some_and(|o| o != "-")
        && let Some(output) = &cli.output
    {
        let output_path = PathBuf::from(output);

        // Validate output path for security
        validate_path(&output_path)?;

        // If the output path exists and is a directory, it's invalid
        if output_path.exists() && output_path.is_dir() {
            return Err(crate::error::ShinkansenError::ValidationError(
                "Cannot use directory output with stdin. Use '-' for stdout or specify a filename"
                    .to_string(),
            ));
        }
    }

    // Check output combinations
    let single_input = using_stdin || cli.inputs.len() == 1;
    let using_stdout = cli.output.as_ref().is_some_and(|o| o == "-");

    if !single_input && using_stdout {
        return Err(crate::error::ShinkansenError::ValidationError(
            "Cannot use '-' (stdout) with multiple inputs".to_string(),
        ));
    }

    if !single_input
        && cli.output.is_some()
        && !using_stdout
        && let Some(output) = &cli.output
    {
        let output_path = PathBuf::from(output);

        // Validate output path for security
        validate_path(&output_path)?;

        if output_path.exists() && !output_path.is_dir() {
            return Err(crate::error::ShinkansenError::ValidationError(
                "Multiple inputs require output to be a directory".to_string(),
            ));
        }
    }

    // For multiple inputs, output must be specified
    if !single_input && cli.output.is_none() {
        return Err(crate::error::ShinkansenError::ValidationError(
            "Multiple inputs require --output directory".to_string(),
        ));
    }

    Ok(())
}

/// Process all inputs
pub fn process_inputs(cli: &Cli, variables: &HashMap<String, minijinja::Value>) -> Result<()> {
    // Check if we're reading from stdin (either explicitly with "-" or implicitly with no inputs)
    let using_stdin = cli.inputs.is_empty() || (cli.inputs.len() == 1 && cli.inputs[0] == "-");

    if using_stdin {
        process_stdin(cli, variables)?;
    } else {
        process_files(cli, variables)?;
    }

    Ok(())
}

fn process_stdin(cli: &Cli, variables: &HashMap<String, minijinja::Value>) -> Result<()> {
    use std::io::{self, Read};

    let mut content = String::new();
    io::stdin()
        .read_to_string(&mut content)
        .with_context(|| "Failed to read from stdin")?;

    // Use a more descriptive template name for better error reporting
    let template_name = "<stdin>";
    let result = crate::rendering::render_template(&content, variables, template_name)?;

    // Determine output destination
    let output_destination = determine_output_destination(cli, true)?;

    // Write to the appropriate output
    write_to_output(&output_destination, Path::new("stdin"), &result, cli)?;

    Ok(())
}

fn process_files(cli: &Cli, variables: &HashMap<String, minijinja::Value>) -> Result<()> {
    let input_files = collect_input_files(cli)?;

    if input_files.is_empty() {
        return Err(crate::error::ShinkansenError::FileSystemError(
            "No files found to process".to_string(),
        ));
    }

    let single_file = input_files.len() == 1;
    let output_destination = determine_output_destination(cli, single_file)?;

    for input_file in &input_files {
        let content = std::fs::read_to_string(input_file)
            .with_context(|| format!("Failed to read file: {:?}", input_file))?;

        let template_name = input_file
            .to_str()
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("file_{}", input_file.display()));

        let result = crate::rendering::render_template(&content, variables, &template_name)?;

        write_to_output(&output_destination, input_file, &result, cli)?;
    }

    Ok(())
}

/// Collect all input files from the provided inputs
fn collect_input_files(cli: &Cli) -> Result<Vec<PathBuf>> {
    let mut input_files = Vec::new();

    for input_str in &cli.inputs {
        let input = PathBuf::from(input_str);
        let normalized_input = normalize_path(&input);

        // Validate input path for security
        validate_path(&normalized_input)?;

        if input.is_file() {
            input_files.push(normalized_input);
        } else if input.is_dir() {
            if cli.recursive {
                for entry in WalkDir::new(&input).into_iter().filter_map(|e| e.ok()) {
                    if entry.file_type().is_file() {
                        let normalized_path = normalize_path(entry.path());
                        input_files.push(normalized_path);
                    }
                }
            } else {
                // Non-recursive: only direct children
                if let Ok(entries) = std::fs::read_dir(&input) {
                    for entry in entries.filter_map(|e| e.ok()) {
                        match entry.file_type() {
                            Ok(file_type) if file_type.is_file() => {
                                let normalized_path = normalize_path(entry.path());
                                input_files.push(normalized_path);
                            }
                            Ok(_) => {}
                            Err(err) => {
                                return Err(crate::error::ShinkansenError::FileSystemError(
                                    format!(
                                        "Failed to get file type for {:?}: {}",
                                        entry.path(),
                                        err
                                    ),
                                ));
                            }
                        }
                    }
                }
            }
        } else {
            return Err(crate::error::ShinkansenError::FileSystemError(format!(
                "Input does not exist: {:?}",
                normalized_input
            )));
        }
    }

    Ok(input_files)
}
