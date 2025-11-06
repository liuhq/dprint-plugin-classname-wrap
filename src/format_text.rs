use crate::{configuration::Configuration, generation::generate};
use dprint_core::{
    configuration::{NewLineKind, resolve_new_line_kind},
    formatting::*,
};
use std::path::Path;

pub struct FormatTextOptions<'a> {
    pub path: &'a Path,
    pub extension: Option<&'a str>,
    pub text: String,
    pub config: &'a Configuration,
}

pub fn format_text(options: FormatTextOptions) -> anyhow::Result<Option<String>> {
    let result = format_text_inner(options.path, &options.text, options.config)?;
    Ok(Some(result))
}

fn format_text_inner(
    path: &Path,
    source_text: &str,
    config: &Configuration,
) -> anyhow::Result<String> {
    let strip_bom_text = strip_bom(source_text);
    Ok(dprint_core::formatting::format(
        || match generate(path, strip_bom_text, config) {
            Ok(result) => result,
            Err(err) => PrintItems::from(String::from(err.to_string())),
        },
        config_to_print_options(strip_bom_text, config),
    ))
}

fn strip_bom(text: &str) -> &str {
    text.strip_prefix("\u{FEFF}").unwrap_or(text)
}

fn config_to_print_options(file_text: &str, _config: &Configuration) -> PrintOptions {
    PrintOptions {
        indent_width: 4,
        max_width: 120,
        use_tabs: false,
        // new_line_text: resolve_new_line_kind(file_text, config.new_line_kind),
        new_line_text: resolve_new_line_kind(file_text, NewLineKind::LineFeed),
    }
}
