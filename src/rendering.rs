use std::collections::HashMap;

use crate::error::{ContextExt, Result};
use minijinja::value::ValueKind;
use minijinja::{Environment, escape_formatter};

/// Format MiniJinja values for output.
///
/// Booleans are rendered as lowercase `true`/`false` to match Rust/JSON/YAML
/// conventions used throughout the CLI. All other values are delegated to the
/// default MiniJinja formatter so auto-escaping behavior is preserved.
fn format_value(
    out: &mut minijinja::Output,
    state: &minijinja::State,
    value: &minijinja::Value,
) -> std::result::Result<(), minijinja::Error> {
    if value.kind() == ValueKind::Bool {
        out.write_str(if value.is_true() { "true" } else { "false" })
            .map_err(minijinja::Error::from)
    } else {
        escape_formatter(out, state, value)
    }
}

/// Validate template content for security and size constraints
fn validate_template_content(content: &str) -> Result<()> {
    // Check for excessively large templates
    if content.len() > 1024 * 1024 {
        // 1MB limit
        return Err(crate::error::ShinkansenError::ValidationError(
            "Template content too large (max 1MB)".to_string(),
        ));
    }

    Ok(())
}

/// Render a template with the given variables
pub fn render_template(
    content: &str,
    variables: &HashMap<String, minijinja::Value>,
    name: &str,
) -> Result<String> {
    // Validate template content before processing
    validate_template_content(content)?;

    let mut env = Environment::new();

    // Configure MiniJinja to treat missing variables as errors
    env.set_undefined_behavior(minijinja::UndefinedBehavior::Strict);

    // Render booleans as lowercase `true`/`false`
    env.set_formatter(format_value);

    env.add_template(name, content)
        .with_context(|| format!("Failed to parse template: {}", name))?;

    let template = env
        .get_template(name)
        .with_context(|| format!("Failed to get template: {}", name))?;

    template
        .render(variables)
        .with_context(|| format!("Failed to render template: {}", name))
}
