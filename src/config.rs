use serde::Deserialize;
use std::collections::HashMap;

use crate::error::Result;

/// Configuration file structure
#[derive(Deserialize, Debug)]
pub struct ConfigFile {
    #[serde(flatten)]
    pub variables: HashMap<String, serde_json::Value>,
}

/// Trait for loading configuration files in different formats
pub trait ConfigLoader {
    /// Load configuration from file content
    fn load_config(&self, content: &str) -> Result<ConfigFile>;

    /// Get the file extensions supported by this loader
    fn supported_extensions(&self) -> &[&'static str];
}

/// JSON configuration loader
pub struct JsonConfigLoader;

impl ConfigLoader for JsonConfigLoader {
    fn load_config(&self, content: &str) -> Result<ConfigFile> {
        serde_json::from_str(content)
            .map_err(|e| crate::error::ShinkansenError::ConfigParseError(e.to_string()))
    }

    fn supported_extensions(&self) -> &[&'static str] {
        &["json"]
    }
}

/// YAML configuration loader
pub struct YamlConfigLoader;

impl ConfigLoader for YamlConfigLoader {
    fn load_config(&self, content: &str) -> Result<ConfigFile> {
        serde_yaml::from_str(content)
            .map_err(|e| crate::error::ShinkansenError::ConfigParseError(e.to_string()))
    }

    fn supported_extensions(&self) -> &[&'static str] {
        &["yaml", "yml"]
    }
}

/// TOML configuration loader
pub struct TomlConfigLoader;

impl ConfigLoader for TomlConfigLoader {
    fn load_config(&self, content: &str) -> Result<ConfigFile> {
        toml::from_str(content)
            .map_err(|e| crate::error::ShinkansenError::ConfigParseError(e.to_string()))
    }

    fn supported_extensions(&self) -> &[&'static str] {
        &["toml"]
    }
}

/// Config loader factory that creates appropriate loaders based on file extension
pub struct ConfigLoaderFactory;

impl ConfigLoaderFactory {
    pub fn create_loader(ext: &str) -> Option<Box<dyn ConfigLoader>> {
        match ext {
            "json" => Some(Box::new(JsonConfigLoader)),
            "yaml" | "yml" => Some(Box::new(YamlConfigLoader)),
            "toml" => Some(Box::new(TomlConfigLoader)),
            _ => None,
        }
    }
}
