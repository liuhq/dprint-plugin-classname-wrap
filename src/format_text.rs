use std::path::Path;

use dprint_core::configuration::resolve_new_line_kind;
use dprint_core::formatting::*;

use super::configuration::Configuration;
use super::generation::generate;

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
    let mut maybe_err: Box<Option<anyhow::Error>> = Box::new(None);
    let strip_bom_text = strip_bom(source_text);
    let result = dprint_core::formatting::format(
        || match generate(path, strip_bom_text, config) {
            Ok(print_items) => print_items,
            Err(err) => {
                maybe_err.replace(err);
                PrintItems::default()
            }
        },
        config_to_print_options(strip_bom_text, config),
    );

    if let Some(err) = maybe_err.take() {
        return Err(err);
    }

    Ok(result)
}

fn strip_bom(text: &str) -> &str {
    text.strip_prefix("\u{FEFF}").unwrap_or(text)
}

fn config_to_print_options(file_text: &str, config: &Configuration) -> PrintOptions {
    PrintOptions {
        indent_width: config.indent_width,
        max_width: config.line_width,
        use_tabs: config.use_tabs,
        new_line_text: resolve_new_line_kind(file_text, config.new_line_kind),
    }
}
