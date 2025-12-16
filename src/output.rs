use std::path::{Path, PathBuf};

use crate::cli::Cli;
use crate::error::{ContextExt, Result};
use crate::platform::{get_max_path_length, normalize_path};

/// Validate a path for security issues
fn validate_path(path: &Path) -> Result<()> {
    let normalized_path = normalize_path(path);

    // Prevent path traversal attacks
    if normalized_path
        .components()
        .any(|comp| matches!(comp, std::path::Component::ParentDir))
    {
        return Err(crate::error::ShinkansenError::SecurityError(
            "Invalid path - contains parent directory references".to_string(),
        ));
    }

    // Check path length and validity using platform-specific limit
    if normalized_path.as_os_str().len() > get_max_path_length() {
        return Err(crate::error::ShinkansenError::ValidationError(format!(
            "Path too long (max {} characters)",
            get_max_path_length()
        )));
    }

    // Check for invalid characters
    if normalized_path
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

/// Output destination enum to represent different output types
#[derive(Debug, PartialEq)]
pub enum OutputDestination {
    /// Output to stdout
    Stdout,
    /// Output to a single file
    SingleFile(PathBuf),
    /// Output to a directory (for multiple files)
    Directory(PathBuf),
}

/// Determine the appropriate output destination based on CLI arguments
pub fn determine_output_destination(cli: &Cli, single_file: bool) -> Result<OutputDestination> {
    if let Some(output_str) = &cli.output {
        if output_str == "-" {
            return Ok(OutputDestination::Stdout);
        }

        let output = PathBuf::from(output_str);

        // Validate output path for security
        validate_path(&output)?;

        if single_file && !output.is_dir() {
            Ok(OutputDestination::SingleFile(output))
        } else {
            Ok(OutputDestination::Directory(output))
        }
    } else if single_file {
        Ok(OutputDestination::Stdout)
    } else {
        Err(crate::error::ShinkansenError::ValidationError(
            "Output directory required for multiple files".to_string(),
        ))
    }
}

/// Write content to the appropriate output destination
pub fn write_to_output(
    destination: &OutputDestination,
    input_file: &Path,
    content: &str,
    cli: &Cli,
) -> Result<()> {
    match destination {
        OutputDestination::Stdout => {
            print!("{}", content);
            Ok(())
        }
        OutputDestination::SingleFile(output_path) => {
            if let Some(parent) = output_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(output_path, content)
                .with_context(|| format!("Failed to write to {:?}", output_path))
        }
        OutputDestination::Directory(output_dir) => {
            // Create output directory if it doesn't exist
            std::fs::create_dir_all(output_dir)?;

            // Preserve directory structure if input was a single directory
            let output_file = determine_output_file_path(output_dir, input_file, cli);

            if let Some(parent) = output_file.parent() {
                std::fs::create_dir_all(parent)?;
            }

            std::fs::write(&output_file, content)
                .with_context(|| format!("Failed to write to {:?}", output_file))
        }
    }
}

/// Determine the output file path for directory output
fn determine_output_file_path(output_dir: &Path, input_file: &Path, cli: &Cli) -> PathBuf {
    let input_path_strs: Vec<PathBuf> = cli.inputs.iter().map(PathBuf::from).collect();

    if input_path_strs.len() == 1
        && input_path_strs[0].is_dir()
        && input_file.starts_with(&input_path_strs[0])
    {
        // Preserve directory structure
        let relative = input_file
            .strip_prefix(&input_path_strs[0])
            .unwrap_or(input_file);
        output_dir.join(relative)
    } else {
        // Just use the filename
        output_dir.join(input_file.file_name().unwrap_or_default())
    }
}

/// Write content directly to stdout
pub fn write_to_stdout(content: &str) {
    print!("{}", content);
}

/// Write content to a specific file path
pub fn write_to_file(path: &Path, content: &str) -> Result<()> {
    // Validate output path for security
    validate_path(path)?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, content).with_context(|| format!("Failed to write to {:?}", path))
}
