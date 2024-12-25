//! Support code

#[cfg(not(target_arch = "wasm32"))]
pub mod error_native;
#[cfg(target_arch = "wasm32")]
pub mod error_wasm;
#[cfg(not(target_arch = "wasm32"))]
mod subscriber;

#[cfg(not(target_arch = "wasm32"))]
pub use error_native::ErrorWrapper;
#[cfg(target_arch = "wasm32")]
pub use error_wasm::ErrorWrapper;
#[cfg(not(target_arch = "wasm32"))]
pub use subscriber::activate_global_default_tracing_subscriber;

pub type Result<T> = std::result::Result<T, ErrorWrapper>;
pub type Error = ErrorWrapper;
