use std::collections::HashMap;

use crate::error::{ContextExt, Result};

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
    variables: &HashMap<String, tera::Value>,
    name: &str,
) -> Result<String> {
    // Validate template content before processing
    validate_template_content(content)?;

    let mut tera = tera::Tera::default();
    tera.add_raw_template(name, content)
        .with_context(|| format!("Failed to parse template: {}", name))?;

    let context = tera::Context::from_serialize(variables)
        .with_context(|| "Failed to create template context")?;

    tera.render(name, &context)
        .with_context(|| format!("Failed to render template: {}", name))
}
