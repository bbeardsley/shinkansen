use std::error::Error;
use std::fmt;
use std::io;

/// Custom error types for Shinkansen
#[derive(Debug)]
pub enum ShinkansenError {
    /// Input/output related errors
    IoError(io::Error),

    /// Template rendering errors
    TemplateError(String),

    /// Configuration file parsing errors
    ConfigParseError(String),

    /// Variable parsing errors
    VariableParseError(String),

    /// Validation errors (input/output combinations)
    ValidationError(String),

    /// File system errors
    FileSystemError(String),

    /// Security-related errors
    SecurityError(String),

    /// Template context creation errors
    ContextError(String),
}

impl fmt::Display for ShinkansenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShinkansenError::IoError(err) => write!(f, "{}", err),
            ShinkansenError::TemplateError(msg) => write!(f, "{}", msg),
            ShinkansenError::ConfigParseError(msg) => write!(f, "{}", msg),
            ShinkansenError::VariableParseError(msg) => write!(f, "{}", msg),
            ShinkansenError::ValidationError(msg) => write!(f, "{}", msg),
            ShinkansenError::FileSystemError(msg) => write!(f, "{}", msg),
            ShinkansenError::SecurityError(msg) => write!(f, "{}", msg),
            ShinkansenError::ContextError(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ShinkansenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ShinkansenError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for ShinkansenError {
    fn from(err: io::Error) -> Self {
        ShinkansenError::IoError(err)
    }
}

impl From<minijinja::Error> for ShinkansenError {
    fn from(err: minijinja::Error) -> Self {
        // Build the full error chain like anyhow does
        let mut full_message = format!("MiniJinja error: {}", err);
        let mut source = err.source();
        let mut level = 1;

        while let Some(src_err) = source {
            full_message.push_str(&format!("\nCaused by:\n    {}: {}", level, src_err));
            source = src_err.source();
            level += 1;
        }

        ShinkansenError::TemplateError(full_message)
    }
}

impl From<serde_json::Error> for ShinkansenError {
    fn from(err: serde_json::Error) -> Self {
        ShinkansenError::ConfigParseError(err.to_string())
    }
}

impl From<serde_yaml::Error> for ShinkansenError {
    fn from(err: serde_yaml::Error) -> Self {
        ShinkansenError::ConfigParseError(err.to_string())
    }
}

impl From<toml::de::Error> for ShinkansenError {
    fn from(err: toml::de::Error) -> Self {
        ShinkansenError::ConfigParseError(err.to_string())
    }
}

/// Result type for Shinkansen operations
pub type Result<T> = std::result::Result<T, ShinkansenError>;

/// Helper trait for adding context to errors
pub trait ContextExt<T> {
    fn with_context<C, F>(self, context: C) -> Result<T>
    where
        C: FnOnce() -> F,
        F: Into<String>;
}

impl<T, E> ContextExt<T> for std::result::Result<T, E>
where
    E: Into<ShinkansenError>,
{
    fn with_context<C, F>(self, _context: C) -> Result<T>
    where
        C: FnOnce() -> F,
        F: Into<String>,
    {
        self.map_err(|e| {
            let error: ShinkansenError = e.into();
            match error {
                ShinkansenError::IoError(err) => {
                    ShinkansenError::FileSystemError(format!("{}", err))
                }
                ShinkansenError::TemplateError(msg) => {
                    ShinkansenError::TemplateError(msg.to_string())
                }
                ShinkansenError::ConfigParseError(msg) => {
                    ShinkansenError::ConfigParseError(msg.to_string())
                }
                ShinkansenError::VariableParseError(msg) => {
                    ShinkansenError::VariableParseError(msg.to_string())
                }
                ShinkansenError::ValidationError(msg) => {
                    ShinkansenError::ValidationError(msg.to_string())
                }
                ShinkansenError::FileSystemError(msg) => {
                    ShinkansenError::FileSystemError(msg.to_string())
                }
                ShinkansenError::SecurityError(msg) => {
                    ShinkansenError::SecurityError(msg.to_string())
                }
                ShinkansenError::ContextError(msg) => {
                    ShinkansenError::ContextError(msg.to_string())
                }
            }
        })
    }
}
