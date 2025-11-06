use dprint_core::{
    configuration::{ConfigKeyMap, GlobalConfiguration},
    generate_plugin_code,
    plugins::{
        CheckConfigUpdatesMessage, ConfigChange, FileMatchingInfo, FormatResult, PluginInfo,
        PluginResolveConfigurationResult, SyncFormatRequest, SyncHostFormatRequest,
        SyncPluginHandler,
    },
};

use crate::format_text::{FormatTextOptions, format_text};

use super::configuration::Configuration;

struct TailwindcssPluginHandler;

impl SyncPluginHandler<Configuration> for TailwindcssPluginHandler {
    fn resolve_config(
        &mut self,
        _config: ConfigKeyMap,
        _global_config: &GlobalConfiguration,
    ) -> PluginResolveConfigurationResult<Configuration> {
        PluginResolveConfigurationResult {
            config: Configuration { line_width: 80 },
            diagnostics: Vec::new(),
            file_matching: FileMatchingInfo {
                file_extensions: vec![
                    String::from("tsx"),
                    String::from("jsx"),
                    String::from("html"),
                ],
                file_names: Vec::new(),
            },
        }
    }

    fn plugin_info(&mut self) -> PluginInfo {
        let name = env!("CARGO_PKG_NAME").to_string();
        let version = env!("CARGO_PKG_VERSION").to_string();
        PluginInfo {
            name,
            version: version.clone(),
            config_key: "tailwindcss".to_string(),
            help_url: "".to_string(),
            config_schema_url: "".to_string(),
            update_url: Some("".to_string()),
        }
    }

    fn license_text(&mut self) -> String {
        std::str::from_utf8(include_bytes!("../LICENSE"))
            .unwrap()
            .into()
    }

    fn check_config_updates(
        &self,
        _message: CheckConfigUpdatesMessage,
    ) -> anyhow::Result<Vec<ConfigChange>> {
        Ok(Vec::new())
    }

    fn format(
        &mut self,
        request: SyncFormatRequest<Configuration>,
        _format_with_host: impl FnMut(SyncHostFormatRequest) -> FormatResult,
    ) -> FormatResult {
        let file_text = String::from_utf8(request.file_bytes)?;
        format_text(FormatTextOptions {
            path: request.file_path,
            extension: None,
            text: file_text,
            config: request.config,
        })
        .map(|maybe_text| maybe_text.map(|t| t.into_bytes()))
    }
}

generate_plugin_code!(TailwindcssPluginHandler, TailwindcssPluginHandler);
