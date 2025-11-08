use std::collections::HashSet;

use dprint_core::configuration::*;
use dprint_core::generate_str_to_from;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SortVersion {
    Alphanumeric,
    V4,
}

generate_str_to_from!(SortVersion, [Alphanumeric, "alphanumeric"], [V4, "v4"]);

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    // required by PrintOptions
    pub use_tabs: bool,
    // required by PrintOptions
    pub new_line_kind: NewLineKind,

    pub tailwind_attributes: HashSet<String>,
    pub tailwind_functions: HashSet<String>,

    pub enable_sort: bool,
    // ignore when `enable_sort` is false
    pub sort_version: SortVersion,

    pub enable_wrap: bool,
    // ignore when `enable_wrap` is false
    pub allow_line_overflow: bool,
    // ignore when `enable_wrap` is false
    pub indent_to_quote: bool,
    // ignore when `enable_wrap` is false
    pub indent_width: u8,
    // ignore when `enable_wrap` is false
    pub line_width_includes_indent: bool,
    // ignore when `enable_wrap` & `line_width_relative_to_indent` are false
    pub line_width: u32,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            use_tabs: false,
            new_line_kind: NewLineKind::Auto,
            tailwind_attributes: HashSet::from_iter(vec![
                String::from("class"),
                String::from("className"),
            ]),
            tailwind_functions: HashSet::new(),
            enable_sort: true,
            sort_version: SortVersion::V4,
            enable_wrap: true,
            allow_line_overflow: false,
            indent_to_quote: true,
            indent_width: 2,
            line_width_includes_indent: false,
            line_width: 120,
        }
    }
}

impl Configuration {
    pub fn with_tailwind_attributes(mut self, patterns: HashSet<String>) -> Self {
        self.tailwind_attributes = patterns;
        self
    }

    pub fn with_tailwind_functions(mut self, patterns: HashSet<String>) -> Self {
        self.tailwind_functions = patterns;
        self
    }

    pub fn with_enable_sort(mut self, enabled: bool) -> Self {
        self.enable_wrap = enabled;
        self
    }

    pub fn with_sort_version(mut self, version: SortVersion) -> Self {
        self.sort_version = version;
        self
    }

    pub fn with_enable_wrap(mut self, enabled: bool) -> Self {
        self.enable_wrap = enabled;
        self
    }

    pub fn with_allow_line_overflow(mut self, enabled: bool) -> Self {
        self.allow_line_overflow = enabled;
        self
    }

    pub fn with_indent_to_quote(mut self, value: bool) -> Self {
        self.indent_to_quote = value;
        self
    }

    pub fn with_indent_width(mut self, width: u8) -> Self {
        self.indent_width = width;
        self
    }

    pub fn with_line_width_includes_indent(mut self, enabled: bool) -> Self {
        self.line_width_includes_indent = enabled;
        self
    }

    pub fn with_line_width(mut self, width: u32) -> Self {
        self.line_width = width;
        self
    }
}

impl Configuration {
    pub fn resolve_config(
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> ResolveConfigurationResult<Self> {
        let mut config = config;
        let mut diagnostics = Vec::new();

        let tailwind_attributes = match get_nullable_vec(
            &mut config,
            "tailwindAttributes",
            |value, i, diagnostics| match value {
                ConfigKeyValue::String(value) => Some(value),
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: format!("tailwindAttributes[{}]", i),
                        message: String::from("Expected array of strings"),
                    });
                    None
                }
            },
            &mut diagnostics,
        ) {
            Some(values) => HashSet::from_iter(values),
            None => HashSet::from_iter(vec![String::from("class"), String::from("className")]),
        };

        let tailwind_functions = HashSet::new();

        let resolved = Self {
            use_tabs: global_config
                .use_tabs
                .unwrap_or(RECOMMENDED_GLOBAL_CONFIGURATION.use_tabs),
            new_line_kind: global_config
                .new_line_kind
                .unwrap_or(RECOMMENDED_GLOBAL_CONFIGURATION.new_line_kind),
            tailwind_attributes,
            tailwind_functions,
            enable_sort: get_value(&mut config, "enableSort", true, &mut diagnostics),
            sort_version: get_value(
                &mut config,
                "sortVersion",
                SortVersion::V4,
                &mut diagnostics,
            ),
            enable_wrap: get_value(&mut config, "enableWrap", true, &mut diagnostics),
            allow_line_overflow: get_value(
                &mut config,
                "allowLineOverflow",
                false,
                &mut diagnostics,
            ),
            indent_to_quote: get_value(&mut config, "indentToQuote", true, &mut diagnostics),
            indent_width: get_value(
                &mut config,
                "indentWidth",
                global_config
                    .indent_width
                    .unwrap_or(RECOMMENDED_GLOBAL_CONFIGURATION.indent_width),
                &mut diagnostics,
            ),
            line_width_includes_indent: get_value(
                &mut config,
                "lineWidthIncludesIndent",
                true,
                &mut diagnostics,
            ),
            line_width: get_value(
                &mut config,
                "lineWidth",
                global_config
                    .line_width
                    .unwrap_or(RECOMMENDED_GLOBAL_CONFIGURATION.line_width),
                &mut diagnostics,
            ),
        };

        ResolveConfigurationResult {
            diagnostics,
            config: resolved,
        }
    }
}
