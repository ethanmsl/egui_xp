//! Library

mod app;
pub mod support;

pub use app::TemplateApp;
#[cfg(not(target_arch = "wasm32"))]
pub use support::active_global_default_tracing_subscriber;
pub use support::{Error, Result};
