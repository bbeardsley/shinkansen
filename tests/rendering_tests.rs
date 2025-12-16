use shinkansen_lib::rendering::render_template;
use std::collections::HashMap;

#[test]
fn test_simple_template_rendering() {
    let template = "Hello, {{ name }}!";
    let mut variables = HashMap::new();
    variables.insert("name".to_string(), "World".into());

    let result = render_template(template, &variables, "test");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello, World!");
}

#[test]
fn test_template_with_filters() {
    let template = "{{ name | upper }}";
    let mut variables = HashMap::new();
    variables.insert("name".to_string(), "hello".into());

    let result = render_template(template, &variables, "test");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "HELLO");
}

#[test]
fn test_template_with_conditionals() {
    let template = r#"
        {% if debug %}
        Debug mode
        {% else %}
        Production mode
        {% endif %}
    "#;

    let mut variables = HashMap::new();
    variables.insert("debug".to_string(), true.into());

    let result = render_template(template, &variables, "test");
    assert!(result.is_ok());
    assert!(result.unwrap().contains("Debug mode"));
}

#[test]
fn test_template_with_loops() {
    let template = r#"
        {% for item in items %}
        - {{ item }}
        {% endfor %}
    "#;

    let mut variables = HashMap::new();
    let items: Vec<tera::Value> = vec!["item1".into(), "item2".into(), "item3".into()];
    variables.insert("items".to_string(), tera::Value::Array(items));

    let result = render_template(template, &variables, "test");
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("- item1"));
    assert!(output.contains("- item2"));
    assert!(output.contains("- item3"));
}

#[test]
fn test_template_with_missing_variable() {
    let template = "Hello, {{ missing_var }}!";
    let variables = HashMap::new();

    let result = render_template(template, &variables, "test");
    assert!(result.is_err());
    let err = result.unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(err_str.contains("missing_var"));
}

#[test]
fn test_stdin_template_name_in_errors() {
    let template = "Hello, {{ missing_var }}!";
    let variables = HashMap::new();

    let result = render_template(template, &variables, "<stdin>");
    assert!(result.is_err());
    let err = result.unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(err_str.contains("<stdin>"));
    assert!(err_str.contains("missing_var"));
}

#[test]
fn test_template_with_default_filter() {
    let template = "{{ missing_var | default(value=\"default\") }}";
    let variables = HashMap::new();

    let result = render_template(template, &variables, "test");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "default");
}

#[test]
fn test_template_with_complex_data() {
    let template = r#"
        Name: {{ user.name }}
        Age: {{ user.age }}
        Active: {{ user.active }}
        Tags: {% for tag in user.tags %}{{ tag }}{% if not loop.last %}, {% endif %}{% endfor %}
    "#;

    let mut variables = HashMap::new();

    let mut user = tera::Map::new();
    user.insert("name".to_string(), tera::Value::String("John".to_string()));
    user.insert("age".to_string(), tera::Value::Number(30.into()));
    user.insert("active".to_string(), tera::Value::Bool(true));

    let tags: Vec<tera::Value> = vec!["rust".into(), "cli".into(), "templates".into()];
    user.insert("tags".to_string(), tera::Value::Array(tags));

    variables.insert("user".to_string(), tera::Value::Object(user));

    let result = render_template(template, &variables, "test");
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("Name: John"));
    assert!(output.contains("Age: 30"));
    assert!(output.contains("Active: true"));
    assert!(output.contains("Tags: rust, cli, templates"));
}

#[test]
fn test_template_with_math_operations() {
    let template = "Result: {{ a + b }}";
    let mut variables = HashMap::new();
    variables.insert("a".to_string(), 10.into());
    variables.insert("b".to_string(), 20.into());

    let result = render_template(template, &variables, "test");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Result: 30");
}

#[test]
fn test_empty_template() {
    let template = "";
    let variables = HashMap::new();

    let result = render_template(template, &variables, "test");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");
}

#[test]
fn test_template_with_comments() {
    let template = r#"
        {# This is a comment #}
        Visible text
        {# Another comment #}
    "#;

    let variables = HashMap::new();
    let result = render_template(template, &variables, "test");
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("Visible text"));
    assert!(!output.contains("This is a comment"));
    assert!(!output.contains("Another comment"));
}
