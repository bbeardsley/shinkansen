use std::collections::HashMap;

use crate::error::{ContextExt, Result};
use minijinja::Environment;

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

    env.add_template(name, content)
        .with_context(|| format!("Failed to parse template: {}", name))?;

    let template = env
        .get_template(name)
        .with_context(|| format!("Failed to get template: {}", name))?;

    template
        .render(variables)
        .with_context(|| format!("Failed to render template: {}", name))
}
