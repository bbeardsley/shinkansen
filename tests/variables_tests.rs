use clap::Parser;
use shinkansen_lib::cli::Cli;
use shinkansen_lib::variables::{
    collect_cli_variables, collect_config_variables, collect_env_variables, collect_variables,
};
use std::collections::HashMap;
use std::env;

#[test]
fn test_cli_variables_parsing() {
    let args = vec![
        "shinkansen",
        "input.txt",
        "-D",
        "name=test",
        "-D",
        "version=1.0.0",
    ];
    let cli = Cli::try_parse_from(args).unwrap();
    let variables = collect_variables(&cli).unwrap();

    assert_eq!(variables.get("name").unwrap(), "test");
    assert_eq!(variables.get("version").unwrap(), "1.0.0");
}

#[test]
fn test_invalid_variable_format() {
    let args = vec!["shinkansen", "input.txt", "-D", "invalid_format"];
    let cli = Cli::try_parse_from(args).unwrap();
    let result = collect_variables(&cli);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Invalid variable format"));
}

#[test]
fn test_env_variables() {
    // Set test environment variables
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { env::set_var("TEST_VAR1", "value1") };
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { env::set_var("TEST_VAR2", "value2") };

    let args = vec!["shinkansen", "input.txt", "--env", "TEST_VAR1,TEST_VAR2"];
    let cli = Cli::try_parse_from(args).unwrap();
    let variables = collect_variables(&cli).unwrap();

    assert_eq!(variables.get("TEST_VAR1").unwrap(), "value1");
    assert_eq!(variables.get("TEST_VAR2").unwrap(), "value2");

    // Clean up
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { env::remove_var("TEST_VAR1") };
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { env::remove_var("TEST_VAR2") };
}

#[test]
fn test_env_variables_numeric_types() {
    // Set test environment variables with numeric values
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { env::set_var("TEST_NUM", "42") };
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { env::set_var("TEST_FLOAT", "3.14") };
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { env::set_var("TEST_BOOL", "true") };
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { env::set_var("TEST_STRING", "hello") };

    let args = vec![
        "shinkansen",
        "input.txt",
        "--env",
        "TEST_NUM,TEST_FLOAT,TEST_BOOL,TEST_STRING",
    ];
    let cli = Cli::try_parse_from(args).unwrap();
    let variables = collect_variables(&cli).unwrap();

    // Test that numeric values are properly converted
    assert_eq!(variables.get("TEST_NUM").unwrap().as_i64().unwrap(), 42);
    assert_eq!(variables.get("TEST_FLOAT").unwrap().as_f64().unwrap(), 3.14);
    assert_eq!(variables.get("TEST_BOOL").unwrap().as_bool().unwrap(), true);
    assert_eq!(
        variables.get("TEST_STRING").unwrap().as_str().unwrap(),
        "hello"
    );

    // Clean up
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { env::remove_var("TEST_NUM") };
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { env::remove_var("TEST_FLOAT") };
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { env::remove_var("TEST_BOOL") };
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { env::remove_var("TEST_STRING") };
}

#[test]
fn test_env_variables_nonexistent() {
    // Ensure variable doesn't exist
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { env::remove_var("NONEXISTENT_VAR") };

    let args = vec!["shinkansen", "input.txt", "--env", "NONEXISTENT_VAR"];
    let cli = Cli::try_parse_from(args).unwrap();
    let variables = collect_variables(&cli).unwrap();

    // Should not contain the nonexistent variable
    assert!(variables.get("NONEXISTENT_VAR").is_none());
}

#[test]
fn test_env_variables_escaped_curly_brackets() {
    // Set environment variable with escaped curly brackets
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { env::set_var("ESCAPED_VAR", "env\\{curly\\}brackets") };

    let args = vec!["shinkansen", "input.txt", "--env", "ESCAPED_VAR"];
    let cli = Cli::try_parse_from(args).unwrap();
    let variables = collect_variables(&cli).unwrap();

    // Should unescape the curly brackets
    assert_eq!(
        variables.get("ESCAPED_VAR").unwrap().as_str().unwrap(),
        "env{curly}brackets"
    );

    // Clean up
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { env::remove_var("ESCAPED_VAR") };
}

#[test]
fn test_variable_precedence() {
    // Set environment variable
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { env::set_var("TEST_VAR", "env_value") };

    // Create a temporary config file
    let config_content = r#"{
        "TEST_VAR": "config_value"
    }"#;

    let config_file = "test_config_precedence.json";
    std::fs::write(config_file, config_content).unwrap();

    let args = vec![
        "shinkansen",
        "input.txt",
        "--env",
        "TEST_VAR",
        "-c",
        config_file,
        "-D",
        "TEST_VAR=cli_value",
    ];
    let cli = Cli::try_parse_from(args).unwrap();
    let variables = collect_variables(&cli).unwrap();

    // CLI should win
    assert_eq!(variables.get("TEST_VAR").unwrap(), "cli_value");

    // Clean up
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { env::remove_var("TEST_VAR") };
    std::fs::remove_file(config_file).ok();
}

#[test]
fn test_config_file_json() {
    let config_content = r#"{
        "string_var": "test",
        "number_var": 42,
        "bool_var": true,
        "array_var": [1, 2, 3],
        "object_var": {"nested": "value"}
    }"#;

    let config_file = "test_config_json.json";
    std::fs::write(config_file, config_content).unwrap();

    let args = vec!["shinkansen", "input.txt", "-c", config_file];
    let cli = Cli::try_parse_from(args).unwrap();
    let variables = collect_variables(&cli).unwrap();

    assert_eq!(
        variables.get("string_var").unwrap().as_str().unwrap(),
        "test"
    );
    assert_eq!(variables.get("number_var").unwrap().as_i64().unwrap(), 42);
    assert_eq!(variables.get("bool_var").unwrap().as_bool().unwrap(), true);

    // Clean up
    std::fs::remove_file(config_file).ok();
}

#[test]
fn test_config_file_unsupported_format() {
    let config_content = r#"string_var = "test""#;

    let config_file = "test_config_unsupported.txt";
    std::fs::write(config_file, config_content).unwrap();

    let args = vec!["shinkansen", "input.txt", "-c", config_file];
    let cli = Cli::try_parse_from(args).unwrap();
    let result = collect_variables(&cli);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Unsupported config file format"));

    // Clean up
    std::fs::remove_file(config_file).ok();
}

#[test]
fn test_empty_variables() {
    let args = vec!["shinkansen", "input.txt"];
    let cli = Cli::try_parse_from(args).unwrap();
    let variables = collect_variables(&cli).unwrap();

    assert!(variables.is_empty());
}

#[test]
fn test_collect_cli_variables_directly() {
    let mut variables = HashMap::new();
    let cli_vars = vec!["name=test".to_string(), "version=1.0.0".to_string()];

    collect_cli_variables(&mut variables, &cli_vars).unwrap();

    assert_eq!(variables.get("name").unwrap(), "test");
    assert_eq!(variables.get("version").unwrap(), "1.0.0");
}

#[test]
fn test_collect_env_variables_directly() {
    // Set test environment variables with unique names
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { env::set_var("TEST_VAR_DIRECT1", "value1") };
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { env::set_var("TEST_VAR_DIRECT2", "value2") };

    let mut variables = HashMap::new();
    let args = vec![
        "shinkansen",
        "input.txt",
        "--env",
        "TEST_VAR_DIRECT1,TEST_VAR_DIRECT2",
    ];
    let cli = Cli::try_parse_from(args).unwrap();

    // Only call collect_env_variables if cli.env is Some
    if cli.env.is_some() {
        collect_env_variables(&mut variables, &cli).unwrap();
    }

    assert_eq!(variables.get("TEST_VAR_DIRECT1").unwrap(), "value1");
    assert_eq!(variables.get("TEST_VAR_DIRECT2").unwrap(), "value2");

    // Clean up
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { env::remove_var("TEST_VAR_DIRECT1") };
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { env::remove_var("TEST_VAR_DIRECT2") };
}

#[test]
fn test_collect_config_variables_directly() {
    let config_content = r#"{
        "string_var": "test",
        "number_var": 42,
        "bool_var": true
    }"#;

    let config_file = "test_config_direct.json";
    std::fs::write(config_file, config_content).unwrap();

    let mut variables = HashMap::new();
    let config_path = std::path::PathBuf::from(config_file);

    collect_config_variables(&mut variables, &config_path).unwrap();

    assert_eq!(
        variables.get("string_var").unwrap().as_str().unwrap(),
        "test"
    );
    assert_eq!(variables.get("number_var").unwrap().as_i64().unwrap(), 42);
    assert_eq!(variables.get("bool_var").unwrap().as_bool().unwrap(), true);

    // Clean up
    std::fs::remove_file(config_file).ok();
}

#[test]
fn test_modular_variable_collection() {
    // Test that we can build variables incrementally using the modular functions
    let mut variables = HashMap::new();

    // Add CLI variables
    let cli_vars = vec!["base=cli_value".to_string()];
    collect_cli_variables(&mut variables, &cli_vars).unwrap();

    assert_eq!(variables.get("base").unwrap(), "cli_value");

    // Add more CLI variables
    let more_cli_vars = vec!["extra=added".to_string()];
    collect_cli_variables(&mut variables, &more_cli_vars).unwrap();

    assert_eq!(variables.get("base").unwrap(), "cli_value");
    assert_eq!(variables.get("extra").unwrap(), "added");
}

#[test]
fn test_cli_variables_with_escaped_characters() {
    let mut variables = HashMap::new();

    // Test escaped comma
    let cli_vars = vec!["a=Hello\\,world".to_string()];
    collect_cli_variables(&mut variables, &cli_vars).unwrap();
    assert_eq!(variables.get("a").unwrap(), "Hello,world");

    // Test escaped equals
    let cli_vars = vec!["b=You\\=rock".to_string()];
    collect_cli_variables(&mut variables, &cli_vars).unwrap();
    assert_eq!(variables.get("b").unwrap(), "You=rock");

    // Test escaped backslash
    let cli_vars = vec!["c=path\\\\to\\\\file".to_string()];
    collect_cli_variables(&mut variables, &cli_vars).unwrap();
    assert_eq!(variables.get("c").unwrap(), "path\\to\\file");

    // Test multiple escaped characters
    let cli_vars = vec!["d=key\\=value\\,more\\\\data".to_string()];
    collect_cli_variables(&mut variables, &cli_vars).unwrap();
    assert_eq!(variables.get("d").unwrap(), "key=value,more\\data");

    // Test trailing backslash
    let cli_vars = vec!["e=path\\\\".to_string()];
    collect_cli_variables(&mut variables, &cli_vars).unwrap();
    assert_eq!(variables.get("e").unwrap(), "path\\");

    // Test the exact example from the requirement
    let cli_vars = vec!["a=Hello\\,world".to_string(), "b=You\\=rock".to_string()];
    let mut variables2 = HashMap::new();
    collect_cli_variables(&mut variables2, &cli_vars).unwrap();
    assert_eq!(variables2.get("a").unwrap(), "Hello,world");
    assert_eq!(variables2.get("b").unwrap(), "You=rock");

    // Test comma-separated variables in a single -D argument
    let cli_vars = vec!["a=Hello\\,world,b=You\\=rock".to_string()];
    let mut variables3 = HashMap::new();
    collect_cli_variables(&mut variables3, &cli_vars).unwrap();
    assert_eq!(variables3.get("a").unwrap(), "Hello,world");
    assert_eq!(variables3.get("b").unwrap(), "You=rock");
}

#[test]
fn test_cli_variables_numeric_types() {
    // Test integer
    let mut variables: HashMap<String, tera::Value> = HashMap::new();
    let cli_vars = vec!["int_var=42".to_string()];
    collect_cli_variables(&mut variables, &cli_vars).unwrap();
    assert_eq!(variables.get("int_var").unwrap().as_i64().unwrap(), 42);

    // Test negative integer
    let mut variables: HashMap<String, tera::Value> = HashMap::new();
    let cli_vars = vec!["neg_int=-10".to_string()];
    collect_cli_variables(&mut variables, &cli_vars).unwrap();
    assert_eq!(variables.get("neg_int").unwrap().as_i64().unwrap(), -10);

    // Test zero
    let mut variables = HashMap::new();
    let cli_vars = vec!["zero_var=0".to_string()];
    collect_cli_variables(&mut variables, &cli_vars).unwrap();
    assert_eq!(variables.get("zero_var").unwrap().as_i64().unwrap(), 0);

    // Test float
    let mut variables: HashMap<String, tera::Value> = HashMap::new();
    let cli_vars = vec!["float_var=3.14".to_string()];
    collect_cli_variables(&mut variables, &cli_vars).unwrap();
    assert_eq!(variables.get("float_var").unwrap().as_f64().unwrap(), 3.14);

    // Test negative float
    let mut variables: HashMap<String, tera::Value> = HashMap::new();
    let cli_vars = vec!["neg_float=-2.5".to_string()];
    collect_cli_variables(&mut variables, &cli_vars).unwrap();
    assert_eq!(variables.get("neg_float").unwrap().as_f64().unwrap(), -2.5);

    // Test boolean true
    let mut variables: HashMap<String, tera::Value> = HashMap::new();
    let cli_vars = vec!["bool_true=true".to_string()];
    collect_cli_variables(&mut variables, &cli_vars).unwrap();
    assert_eq!(variables.get("bool_true").unwrap().as_bool().unwrap(), true);

    // Test boolean false
    let mut variables: HashMap<String, tera::Value> = HashMap::new();
    let cli_vars = vec!["bool_false=false".to_string()];
    collect_cli_variables(&mut variables, &cli_vars).unwrap();
    assert_eq!(
        variables.get("bool_false").unwrap().as_bool().unwrap(),
        false
    );

    // Test string (non-numeric)
    let mut variables: HashMap<String, tera::Value> = HashMap::new();
    let cli_vars = vec!["string_var=hello".to_string()];
    collect_cli_variables(&mut variables, &cli_vars).unwrap();
    assert_eq!(
        variables.get("string_var").unwrap().as_str().unwrap(),
        "hello"
    );

    // Test mixed types in one command
    let mut variables2: HashMap<String, tera::Value> = HashMap::new();
    let cli_vars = vec!["int=42,float=1.5,bool=true,text=test".to_string()];
    collect_cli_variables(&mut variables2, &cli_vars).unwrap();
    assert_eq!(variables2.get("int").unwrap().as_i64().unwrap(), 42);
    assert_eq!(variables2.get("float").unwrap().as_f64().unwrap(), 1.5);
    assert_eq!(variables2.get("bool").unwrap().as_bool().unwrap(), true);
    assert_eq!(variables2.get("text").unwrap().as_str().unwrap(), "test");
}

#[test]
fn test_cli_variables_json_arrays_and_objects() {
    // Test JSON array parsing
    let mut variables: HashMap<String, tera::Value> = HashMap::new();
    let cli_vars = vec!["items=[1,2,3]".to_string()];
    collect_cli_variables(&mut variables, &cli_vars).unwrap();

    if let Some(tera::Value::Array(arr)) = variables.get("items") {
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0].as_i64().unwrap(), 1);
        assert_eq!(arr[1].as_i64().unwrap(), 2);
        assert_eq!(arr[2].as_i64().unwrap(), 3);
    } else {
        panic!("items is not an array");
    }

    // Test JSON array with strings
    let mut variables2 = HashMap::new();
    let cli_vars = vec!["items=[\"hello\",\"world\"]".to_string()];
    collect_cli_variables(&mut variables2, &cli_vars).unwrap();

    if let Some(tera::Value::Array(arr)) = variables2.get("items") {
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0].as_str().unwrap(), "hello");
        assert_eq!(arr[1].as_str().unwrap(), "world");
    } else {
        panic!("items is not an array");
    }

    // Test JSON object parsing
    let mut variables3 = HashMap::new();
    let cli_vars = vec!["obj={\"key\":\"value\"}".to_string()];
    collect_cli_variables(&mut variables3, &cli_vars).unwrap();

    if let Some(tera::Value::Object(obj)) = variables3.get("obj") {
        assert_eq!(obj.get("key").unwrap().as_str().unwrap(), "value");
    } else {
        panic!("obj is not an object");
    }

    // Test nested JSON structures
    let mut variables4 = HashMap::new();
    let cli_vars = vec!["data={\"nested\":{\"array\":[1,2,3]}}".to_string()];
    collect_cli_variables(&mut variables4, &cli_vars).unwrap();

    if let Some(tera::Value::Object(obj)) = variables4.get("data") {
        if let Some(tera::Value::Object(nested)) = obj.get("nested") {
            if let Some(tera::Value::Array(arr)) = nested.get("array") {
                assert_eq!(arr.len(), 3);
                assert_eq!(arr[0].as_i64().unwrap(), 1);
            } else {
                panic!("nested.array is not an array");
            }
        } else {
            panic!("data.nested is not an object");
        }
    } else {
        panic!("data is not an object");
    }

    // Test mixed JSON and regular values in one command
    let mut variables5: HashMap<String, tera::Value> = HashMap::new();
    let cli_vars = vec!["simple=hello,arr=[1,2],obj={\"k\":\"v\"}".to_string()];
    collect_cli_variables(&mut variables5, &cli_vars).unwrap();

    assert_eq!(variables5.get("simple").unwrap().as_str().unwrap(), "hello");

    if let Some(tera::Value::Array(arr)) = variables5.get("arr") {
        assert_eq!(arr.len(), 2);
    } else {
        panic!("arr is not an array");
    }

    if let Some(tera::Value::Object(obj)) = variables5.get("obj") {
        assert_eq!(obj.get("k").unwrap().as_str().unwrap(), "v");
    } else {
        panic!("obj is not an object");
    }

    // Test that commas inside JSON arrays don't split variables
    let mut variables6: HashMap<String, tera::Value> = HashMap::new();
    let cli_vars = vec!["a=1,items=[1,2,3],b=2".to_string()];
    collect_cli_variables(&mut variables6, &cli_vars).unwrap();

    assert_eq!(variables6.get("a").unwrap().as_i64().unwrap(), 1);
    assert_eq!(variables6.get("b").unwrap().as_i64().unwrap(), 2);

    if let Some(tera::Value::Array(arr)) = variables6.get("items") {
        assert_eq!(arr.len(), 3);
    } else {
        panic!("items is not an array");
    }
}

#[test]
fn test_cli_variables_escaped_brackets() {
    // Test escaped square brackets in variable values
    let mut variables: HashMap<String, tera::Value> = HashMap::new();
    let cli_vars = vec![r"value=hello\[world\]".to_string()];
    collect_cli_variables(&mut variables, &cli_vars).unwrap();

    assert_eq!(
        variables.get("value").unwrap().as_str().unwrap(),
        "hello[world]"
    );

    // Test mixed escaped characters
    let mut variables2: HashMap<String, tera::Value> = HashMap::new();
    let cli_vars = vec![r"value=test\[1\]\,more".to_string()];
    collect_cli_variables(&mut variables2, &cli_vars).unwrap();

    assert_eq!(
        variables2.get("value").unwrap().as_str().unwrap(),
        "test[1],more"
    );

    // Test multiple escaped brackets
    let mut variables3: HashMap<String, tera::Value> = HashMap::new();
    let cli_vars = vec![r"value=\[start\]middle\[end\]".to_string()];
    collect_cli_variables(&mut variables3, &cli_vars).unwrap();

    assert_eq!(
        variables3.get("value").unwrap().as_str().unwrap(),
        "[start]middle[end]"
    );

    // Test escaped brackets don't interfere with JSON arrays
    let mut variables4 = HashMap::new();
    let cli_vars = vec![r"arr=[1,2,3],escaped=test\[bracket\]".to_string()];
    collect_cli_variables(&mut variables4, &cli_vars).unwrap();

    if let Some(tera::Value::Array(arr)) = variables4.get("arr") {
        assert_eq!(arr.len(), 3);
    } else {
        panic!("arr is not an array");
    }

    assert_eq!(
        variables4.get("escaped").unwrap().as_str().unwrap(),
        "test[bracket]"
    );
}

#[test]
fn test_cli_variables_escaped_curly_brackets() {
    // Test escaped curly brackets in variable values
    let mut variables: HashMap<String, tera::Value> = HashMap::new();
    let cli_vars = vec![r"value=hello\{world\}".to_string()];
    collect_cli_variables(&mut variables, &cli_vars).unwrap();

    assert_eq!(
        variables.get("value").unwrap().as_str().unwrap(),
        "hello{world}"
    );

    // Test mixed escaped characters including curly brackets
    let mut variables2: HashMap<String, tera::Value> = HashMap::new();
    let cli_vars = vec![r"value=test\{1\}\,more\=data".to_string()];
    collect_cli_variables(&mut variables2, &cli_vars).unwrap();

    assert_eq!(
        variables2.get("value").unwrap().as_str().unwrap(),
        "test{1},more=data"
    );

    // Test multiple escaped curly brackets
    let mut variables3: HashMap<String, tera::Value> = HashMap::new();
    let cli_vars = vec![r"value=\{start\}middle\{end\}".to_string()];
    collect_cli_variables(&mut variables3, &cli_vars).unwrap();

    assert_eq!(
        variables3.get("value").unwrap().as_str().unwrap(),
        "{start}middle{end}"
    );
}

#[test]
fn test_cli_variables_nested_objects() {
    // Test simple nested object
    let mut variables = HashMap::new();
    let cli_vars = vec!["foo.bar=42".to_string()];
    collect_cli_variables(&mut variables, &cli_vars).unwrap();

    // Check that foo exists and is an object
    let foo_value: &tera::Value = variables.get("foo").unwrap();
    assert!(foo_value.is_object());

    // Check that foo.bar equals 42
    if let Some(tera::Value::Object(obj)) = variables.get("foo") {
        if let Some(bar_value) = obj.get("bar") {
            assert_eq!(bar_value.as_i64().unwrap(), 42);
        } else {
            panic!("bar key not found in foo object");
        }
    } else {
        panic!("foo is not an object");
    }

    // Test multiple nested properties in one command
    let mut variables2 = HashMap::new();
    let cli_vars = vec!["foo.bar=42,foo.baz=true".to_string()];
    collect_cli_variables(&mut variables2, &cli_vars).unwrap();

    if let Some(tera::Value::Object(obj)) = variables2.get("foo") {
        assert_eq!(obj.get("bar").unwrap().as_i64().unwrap(), 42);
        assert_eq!(obj.get("baz").unwrap().as_bool().unwrap(), true);
    } else {
        panic!("foo is not an object");
    }

    // Test deeply nested objects
    let mut variables3 = HashMap::new();
    let cli_vars = vec!["foo.bar.baz.qux=deep_value".to_string()];
    collect_cli_variables(&mut variables3, &cli_vars).unwrap();

    if let Some(tera::Value::Object(foo_obj)) = variables3.get("foo") {
        if let Some(tera::Value::Object(bar_obj)) = foo_obj.get("bar") {
            if let Some(tera::Value::Object(baz_obj)) = bar_obj.get("baz") {
                assert_eq!(baz_obj.get("qux").unwrap().as_str().unwrap(), "deep_value");
            } else {
                panic!("baz is not an object");
            }
        } else {
            panic!("bar is not an object");
        }
    } else {
        panic!("foo is not an object");
    }

    // Test merging with existing nested structures
    let mut variables4 = HashMap::new();
    let cli_vars1 = vec!["foo.bar=first".to_string()];
    collect_cli_variables(&mut variables4, &cli_vars1).unwrap();

    let cli_vars2 = vec!["foo.baz=second".to_string()];
    collect_cli_variables(&mut variables4, &cli_vars2).unwrap();

    if let Some(tera::Value::Object(obj)) = variables4.get("foo") {
        assert_eq!(obj.get("bar").unwrap().as_str().unwrap(), "first");
        assert_eq!(obj.get("baz").unwrap().as_str().unwrap(), "second");
    } else {
        panic!("foo is not an object");
    }

    // Test mixed nested and non-nested variables
    let mut variables5: HashMap<String, tera::Value> = HashMap::new();
    let cli_vars = vec!["simple=hello,foo.bar=world".to_string()];
    collect_cli_variables(&mut variables5, &cli_vars).unwrap();

    assert_eq!(variables5.get("simple").unwrap().as_str().unwrap(), "hello");

    if let Some(tera::Value::Object(obj)) = variables5.get("foo") {
        assert_eq!(obj.get("bar").unwrap().as_str().unwrap(), "world");
    } else {
        panic!("foo is not an object");
    }
}
