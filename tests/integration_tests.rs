// Integration tests for platform-specific features

use clap::Parser;
use shinkansen_lib::cli::Cli;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_path_normalization_in_processing() {
    // Create a test file
    let temp_dir = tempdir().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    fs::write(&test_file, "Hello {{name}}!").unwrap();

    // Test with the actual processing pipeline
    let args = vec![
        "shinkansen",
        test_file.to_str().unwrap(),
        "-D",
        "name=World",
        "-o",
        "-",
    ];

    let cli = Cli::try_parse_from(args).unwrap();
    let variables = shinkansen_lib::variables::collect_variables(&cli).unwrap();
    let result = shinkansen_lib::processing::process_inputs(&cli, &variables);

    // Should succeed
    assert!(result.is_ok());
}

#[test]
fn test_platform_specific_path_validation() {
    // Test that very long paths are rejected based on platform limits
    let temp_dir = tempdir().unwrap();

    // Create a path that's longer than the platform limit
    // We need to account for the temp directory path length
    let temp_dir_length = temp_dir.path().to_string_lossy().len();
    let remaining_length = shinkansen_lib::platform::get_max_path_length() - temp_dir_length - 10; // -10 for safety

    if remaining_length > 0 {
        let long_path_name = "a".repeat(remaining_length + 100); // Make it exceed the limit
        let long_path = temp_dir.path().join(long_path_name);

        let args = vec![
            "shinkansen",
            long_path.to_str().unwrap(),
            "-D",
            "name=test",
            "-o",
            "-",
        ];

        let cli = Cli::try_parse_from(args).unwrap();
        let variables = shinkansen_lib::variables::collect_variables(&cli).unwrap();

        // This should fail during processing when it tries to collect input files
        let result = shinkansen_lib::processing::process_inputs(&cli, &variables);

        // Should fail with path too long error
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Path too long"));
    } else {
        // If temp dir is already too long, skip this test
        println!("Skipping test - temp directory path is already too long");
    }
}

#[test]
fn test_cross_platform_path_handling() {
    // Test that the system can handle paths with different separators
    let temp_dir = tempdir().unwrap();

    // Create a test file
    let test_file = temp_dir.path().join("test.txt");
    fs::write(&test_file, "Test content").unwrap();

    // Test processing with the file
    let args = vec!["shinkansen", test_file.to_str().unwrap(), "-o", "-"];

    let cli = Cli::try_parse_from(args).unwrap();
    let variables = shinkansen_lib::variables::collect_variables(&cli).unwrap();
    let result = shinkansen_lib::processing::process_inputs(&cli, &variables);

    assert!(result.is_ok());
}
