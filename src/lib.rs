//! Library

mod app;
pub mod support;

pub use app::TemplateApp;
#[cfg(not(target_arch = "wasm32"))]
pub use support::activate_global_default_tracing_subscriber;
#[cfg(not(target_arch = "wasm32"))]
pub use support::{Error, Result};
