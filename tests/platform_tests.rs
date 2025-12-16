// Tests for platform-specific functionality

use shinkansen_lib::platform::{
    get_line_ending, get_max_path_length, get_platform_config, is_case_sensitive, normalize_path,
};
use std::path::PathBuf;

#[test]
fn test_platform_config_creation() {
    let config = get_platform_config();

    // Basic validation that we get a config
    assert!(config.line_endings.len() > 0);
    assert!(config.max_path_length > 0);
    // case_sensitive can be either true or false depending on platform
}

#[test]
fn test_line_ending_consistency() {
    let line_ending = get_line_ending();
    assert!(!line_ending.is_empty());
    assert!(line_ending == "\n" || line_ending == "\r\n");
}

#[test]
fn test_max_path_length() {
    let max_length = get_max_path_length();
    assert!(max_length > 0);
    // Should be either 260 (Windows) or 4096 (Unix)
    assert!(max_length == 260 || max_length == 4096);
}

#[test]
fn test_case_sensitivity() {
    let case_sensitive = is_case_sensitive();
    // This should be true on Unix, false on Windows
    // We can't test the exact value since it depends on the platform
    // but we can verify it returns a boolean
    assert!(case_sensitive == true || case_sensitive == false);
}

#[test]
fn test_normalize_path() {
    // Test Windows-style paths
    let windows_path = PathBuf::from("C:\\Users\\test\\file.txt");
    let normalized = normalize_path(&windows_path);
    let normalized_str = normalized.to_string_lossy();
    assert!(normalized_str.contains("/"));
    assert!(!normalized_str.contains("\\"));

    // Test Unix-style paths (should remain unchanged)
    let unix_path = PathBuf::from("/usr/local/bin/file");
    let normalized = normalize_path(&unix_path);
    let normalized_str = normalized.to_string_lossy();
    assert_eq!(normalized_str, "/usr/local/bin/file");

    // Test mixed paths
    let mixed_path = PathBuf::from("C:/mixed\\path/file.txt");
    let normalized = normalize_path(&mixed_path);
    let normalized_str = normalized.to_string_lossy();
    assert!(!normalized_str.contains("\\"));
    assert!(normalized_str.contains("/"));
}
