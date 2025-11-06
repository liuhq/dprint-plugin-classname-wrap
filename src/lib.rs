pub mod configuration;
mod format_text;
mod generation;

pub use format_text::FormatTextOptions;
pub use format_text::format_text;

#[cfg(feature = "wasm")]
#[cfg(target_arch = "wasm32")]
#[cfg(target_os = "unknown")]
mod wasm_plugin;

#[cfg(feature = "wasm")]
#[cfg(target_arch = "wasm32")]
#[cfg(target_os = "unknown")]
pub use wasm_plugin::*;
