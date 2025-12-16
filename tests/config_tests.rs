use shinkansen_lib::config::{
    ConfigLoader, ConfigLoaderFactory, JsonConfigLoader, TomlConfigLoader, YamlConfigLoader,
};

#[test]
fn test_json_config_loader() {
    let loader = JsonConfigLoader;
    let json_content = r#"{
        "name": "test",
        "version": "1.0.0",
        "enabled": true
    }"#;

    let config = loader.load_config(json_content).unwrap();
    assert_eq!(config.variables.get("name").unwrap(), "test");
    assert_eq!(config.variables.get("version").unwrap(), "1.0.0");
    assert_eq!(config.variables.get("enabled").unwrap(), true);
}

#[test]
fn test_yaml_config_loader() {
    let loader = YamlConfigLoader;
    let yaml_content = "
name: test
version: 1.0.0
enabled: true
";

    let config = loader.load_config(yaml_content).unwrap();
    assert_eq!(config.variables.get("name").unwrap(), "test");
    assert_eq!(config.variables.get("version").unwrap(), "1.0.0");
    assert_eq!(config.variables.get("enabled").unwrap(), true);
}

#[test]
fn test_toml_config_loader() {
    let loader = TomlConfigLoader;
    let toml_content = "
name = 'test'
version = '1.0.0'
enabled = true
";

    let config = loader.load_config(toml_content).unwrap();
    assert_eq!(config.variables.get("name").unwrap(), "test");
    assert_eq!(config.variables.get("version").unwrap(), "1.0.0");
    assert_eq!(config.variables.get("enabled").unwrap(), true);
}

#[test]
fn test_config_loader_factory() {
    // Test JSON loader creation
    let json_loader = ConfigLoaderFactory::create_loader("json").unwrap();
    assert_eq!(json_loader.supported_extensions(), &["json"]);

    // Test YAML loader creation
    let yaml_loader = ConfigLoaderFactory::create_loader("yaml").unwrap();
    assert_eq!(yaml_loader.supported_extensions(), &["yaml", "yml"]);

    // Test YML loader creation (should be same as YAML)
    let yml_loader = ConfigLoaderFactory::create_loader("yml").unwrap();
    assert_eq!(yml_loader.supported_extensions(), &["yaml", "yml"]);

    // Test TOML loader creation
    let toml_loader = ConfigLoaderFactory::create_loader("toml").unwrap();
    assert_eq!(toml_loader.supported_extensions(), &["toml"]);

    // Test unsupported format
    assert!(ConfigLoaderFactory::create_loader("xml").is_none());
    assert!(ConfigLoaderFactory::create_loader("ini").is_none());
}

#[test]
fn test_json_config_loader_with_complex_data() {
    let loader = JsonConfigLoader;
    let json_content = r#"{
        "simple": "value",
        "number": 42,
        "float": 3.14,
        "boolean": false,
        "null": null,
        "array": [1, 2, 3],
        "object": {
            "nested": "value"
        }
    }"#;

    let config = loader.load_config(json_content).unwrap();
    assert_eq!(config.variables.len(), 7);
    assert!(config.variables.contains_key("simple"));
    assert!(config.variables.contains_key("number"));
    assert!(config.variables.contains_key("float"));
    assert!(config.variables.contains_key("boolean"));
    assert!(config.variables.contains_key("null"));
    assert!(config.variables.contains_key("array"));
    assert!(config.variables.contains_key("object"));
}

#[test]
fn test_invalid_json_config() {
    let loader = JsonConfigLoader;
    let invalid_json = "{\"name\": \"test\", \"version\": \"1.0.0\",};";

    let result = loader.load_config(invalid_json);
    assert!(result.is_err());
}

#[test]
fn test_invalid_yaml_config() {
    let loader = YamlConfigLoader;
    let invalid_yaml = "name: test\nversion: 1.0.0\n  invalid: :";

    let result = loader.load_config(invalid_yaml);
    assert!(result.is_err());
}

#[test]
fn test_invalid_toml_config() {
    let loader = TomlConfigLoader;
    let invalid_toml = "name = 'test'\nversion = '1.0.0'\ninvalid = ";

    let result = loader.load_config(invalid_toml);
    assert!(result.is_err());
}
