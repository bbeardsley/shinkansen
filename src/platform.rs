// Platform-specific configuration and utilities
// This module handles cross-platform differences in path handling, line endings, etc.

use std::path::{Path, PathBuf};

/// Platform-specific configuration
#[derive(Debug, Clone)]
pub struct PlatformConfig {
    /// Line endings for this platform
    pub line_endings: &'static str,
    /// Maximum path length for this platform
    pub max_path_length: usize,
    /// Whether the filesystem is case-sensitive
    pub case_sensitive: bool,
}

/// Get platform-specific configuration
pub fn get_platform_config() -> PlatformConfig {
    #[cfg(windows)]
    {
        PlatformConfig {
            line_endings: "\r\n",
            max_path_length: 260,
            case_sensitive: false,
        }
    }

    #[cfg(unix)]
    {
        PlatformConfig {
            line_endings: "\n",
            max_path_length: 4096,
            case_sensitive: true,
        }
    }

    #[cfg(not(any(windows, unix)))]
    {
        // Default configuration for other platforms
        PlatformConfig {
            line_endings: "\n",
            max_path_length: 4096,
            case_sensitive: true,
        }
    }
}

/// Normalize a path to use forward slashes consistently
/// Normalize a path to use forward slashes consistently
/// This is a more robust version that can be used across the codebase
pub fn normalize_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let path_str = path.as_ref().to_string_lossy().replace('\\', "/");
    PathBuf::from(path_str)
}

/// Get the appropriate line ending for the current platform
pub fn get_line_ending() -> &'static str {
    get_platform_config().line_endings
}

/// Check if the filesystem is case-sensitive
pub fn is_case_sensitive() -> bool {
    get_platform_config().case_sensitive
}

/// Get maximum path length for the current platform
pub fn get_max_path_length() -> usize {
    get_platform_config().max_path_length
}
