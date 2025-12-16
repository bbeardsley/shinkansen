use std::path::PathBuf;
use tempfile::tempdir;

use shinkansen_lib::cli::Cli;
use shinkansen_lib::output::{OutputDestination, determine_output_destination, write_to_output};

#[test]
fn test_determine_output_destination_stdout() {
    let cli = Cli {
        command: None,
        inputs: vec!["-".to_string()],
        output: Some("-".to_string()),
        recursive: false,
        variables: vec![],
        config: None,
        env: None,
    };

    let result = determine_output_destination(&cli, true).unwrap();
    assert_eq!(result, OutputDestination::Stdout);
}

#[test]
fn test_determine_output_destination_single_file() {
    let cli = Cli {
        command: None,
        inputs: vec!["input.txt".to_string()],
        output: Some("output.txt".to_string()),
        recursive: false,
        variables: vec![],
        config: None,
        env: None,
    };

    let result = determine_output_destination(&cli, true).unwrap();
    match result {
        OutputDestination::SingleFile(path) => {
            assert_eq!(path, PathBuf::from("output.txt"));
        }
        _ => panic!("Expected SingleFile output destination"),
    }
}

#[test]
fn test_determine_output_destination_directory() {
    let cli = Cli {
        command: None,
        inputs: vec!["input1.txt".to_string(), "input2.txt".to_string()],
        output: Some("output_dir".to_string()),
        recursive: false,
        variables: vec![],
        config: None,
        env: None,
    };

    let result = determine_output_destination(&cli, false).unwrap();
    match result {
        OutputDestination::Directory(path) => {
            assert_eq!(path, PathBuf::from("output_dir"));
        }
        _ => panic!("Expected Directory output destination"),
    }
}

#[test]
fn test_determine_output_destination_default_stdout() {
    let cli = Cli {
        command: None,
        inputs: vec!["input.txt".to_string()],
        output: None,
        recursive: false,
        variables: vec![],
        config: None,
        env: None,
    };

    let result = determine_output_destination(&cli, true).unwrap();
    assert_eq!(result, OutputDestination::Stdout);
}

#[test]
fn test_determine_output_destination_error() {
    let cli = Cli {
        command: None,
        inputs: vec!["input1.txt".to_string(), "input2.txt".to_string()],
        output: None,
        recursive: false,
        variables: vec![],
        config: None,
        env: None,
    };

    let result = determine_output_destination(&cli, false);
    assert!(result.is_err());
}

#[test]
fn test_write_to_stdout() {
    let destination = OutputDestination::Stdout;
    let input_path = PathBuf::from("test.txt");
    let content = "Hello World!";

    let cli = Cli {
        command: None,
        inputs: vec!["test.txt".to_string()],
        output: Some("-".to_string()),
        recursive: false,
        variables: vec![],
        config: None,
        env: None,
    };

    // This should not panic and should write to stdout
    let result = write_to_output(&destination, &input_path, content, &cli);
    assert!(result.is_ok());
}

#[test]
fn test_write_to_single_file() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("output.txt");

    let destination = OutputDestination::SingleFile(output_path.clone());
    let input_path = PathBuf::from("test.txt");
    let content = "Hello World!";

    let cli = Cli {
        command: None,
        inputs: vec!["test.txt".to_string()],
        output: Some(output_path.to_str().unwrap().to_string()),
        recursive: false,
        variables: vec![],
        config: None,
        env: None,
    };

    let result = write_to_output(&destination, &input_path, content, &cli);
    assert!(result.is_ok());

    // Verify the file was created and contains the correct content
    let written_content = std::fs::read_to_string(output_path).unwrap();
    assert_eq!(written_content, content);
}

#[test]
fn test_write_to_directory() {
    let temp_dir = tempdir().unwrap();
    let output_dir = temp_dir.path().join("output");

    let destination = OutputDestination::Directory(output_dir.clone());
    let input_path = PathBuf::from("test.txt");
    let content = "Hello World!";

    let cli = Cli {
        command: None,
        inputs: vec!["test.txt".to_string()],
        output: Some(output_dir.to_str().unwrap().to_string()),
        recursive: false,
        variables: vec![],
        config: None,
        env: None,
    };

    let result = write_to_output(&destination, &input_path, content, &cli);
    assert!(result.is_ok());

    // Verify the file was created in the directory
    let output_file = output_dir.join("test.txt");
    let written_content = std::fs::read_to_string(output_file).unwrap();
    assert_eq!(written_content, content);
}
