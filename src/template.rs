use crate::constants::{DEFAULT_TEMPLATE, DEFAULT_TEMPLATE_NAME};
use crate::document::Document;
use crate::error::{Result, TdyError};
use crate::ext::DateFmtExt;
use minijinja::{context, Environment};

pub fn render_document(document: &Document) -> Result<String> {
    let mut env = Environment::new();
    env.add_template(DEFAULT_TEMPLATE_NAME, DEFAULT_TEMPLATE)
        .map_err(|e| TdyError::Template(e.to_string()))?;

    env.get_template(DEFAULT_TEMPLATE_NAME)
        .map_err(|e| TdyError::Template(e.to_string()))?
        .render(context!(
            namespace => &document.namespace,
            title => &document.title,
            year => document.date.format("%Y").to_string(),
            month => document.date.format("%m").to_string(),
            day => document.date.format("%d").to_string(),
            date => document.date.ymd()
        ))
        .map_err(|e| TdyError::Template(e.to_string()))
}
