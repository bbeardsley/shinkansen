use clap::Parser;
use shinkansen_lib::cli::Cli;
use shinkansen_lib::processing::validate_args;

#[test]
fn test_validate_no_input_defaults_to_stdin() {
    let args = vec!["shinkansen"];
    let cli = Cli::try_parse_from(args).unwrap();
    let result = validate_args(&cli);

    // Should now succeed - no input defaults to stdin
    assert!(result.is_ok());
}

#[test]
fn test_validate_stdin_with_files() {
    let args = vec!["shinkansen", "-", "file.txt"];
    let cli = Cli::try_parse_from(args).unwrap();
    let result = validate_args(&cli);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        err.to_string()
            .contains("Cannot use '-' (stdin) with other input files")
    );
}

#[test]
fn test_validate_stdout_with_multiple_inputs() {
    let args = vec!["shinkansen", "file1.txt", "file2.txt", "-o", "-"];
    let cli = Cli::try_parse_from(args).unwrap();
    let result = validate_args(&cli);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        err.to_string()
            .contains("Cannot use '-' (stdout) with multiple inputs")
    );
}

#[test]
fn test_validate_multiple_inputs_no_output() {
    let args = vec!["shinkansen", "file1.txt", "file2.txt"];
    let cli = Cli::try_parse_from(args).unwrap();
    let result = validate_args(&cli);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        err.to_string()
            .contains("Multiple inputs require --output directory")
    );
}

#[test]
fn test_validate_multiple_inputs_output_not_directory() {
    // Create a regular file to use as output
    std::fs::write("test_output.txt", "existing content").unwrap();

    let args = vec![
        "shinkansen",
        "file1.txt",
        "file2.txt",
        "-o",
        "test_output.txt",
    ];
    let cli = Cli::try_parse_from(args).unwrap();
    let result = validate_args(&cli);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        err.to_string()
            .contains("Multiple inputs require output to be a directory")
    );

    // Clean up
    std::fs::remove_file("test_output.txt").ok();
}

#[test]
fn test_validate_valid_single_file() {
    let args = vec!["shinkansen", "input.txt"];
    let cli = Cli::try_parse_from(args).unwrap();
    let result = validate_args(&cli);

    assert!(result.is_ok());
}

#[test]
fn test_validate_valid_stdin() {
    let args = vec!["shinkansen", "-"];
    let cli = Cli::try_parse_from(args).unwrap();
    let result = validate_args(&cli);

    assert!(result.is_ok());
}

#[test]
fn test_validate_valid_stdin_stdout() {
    let args = vec!["shinkansen", "-", "-o", "-"];
    let cli = Cli::try_parse_from(args).unwrap();
    let result = validate_args(&cli);

    assert!(result.is_ok());
}

#[test]
fn test_validate_stdin_with_directory_output() {
    // Create a test directory
    std::fs::create_dir_all("test_output_dir_stdin").unwrap();

    // Verify directory exists
    assert!(std::path::Path::new("test_output_dir_stdin").exists());
    assert!(std::path::Path::new("test_output_dir_stdin").is_dir());

    let args = vec!["shinkansen", "-", "-o", "test_output_dir_stdin"];
    let cli = Cli::try_parse_from(args).unwrap();
    let result = validate_args(&cli);

    assert!(result.is_err(), "Expected error but got: {:?}", result);
    let err = result.unwrap_err();
    assert!(
        err.to_string()
            .contains("Cannot use directory output with stdin")
    );

    // Clean up
    std::fs::remove_dir("test_output_dir_stdin").ok();
}

#[test]
fn test_validate_valid_multiple_files_with_directory() {
    // Create a test directory
    std::fs::create_dir_all("test_output_dir").unwrap();

    let args = vec![
        "shinkansen",
        "file1.txt",
        "file2.txt",
        "-o",
        "test_output_dir",
    ];
    let cli = Cli::try_parse_from(args).unwrap();
    let result = validate_args(&cli);

    assert!(result.is_ok());

    // Clean up
    std::fs::remove_dir("test_output_dir").ok();
}

#[test]
fn test_validate_valid_single_file_with_output() {
    let args = vec!["shinkansen", "input.txt", "-o", "output.txt"];
    let cli = Cli::try_parse_from(args).unwrap();
    let result = validate_args(&cli);

    assert!(result.is_ok());
}
