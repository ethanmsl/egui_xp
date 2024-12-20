//! Tracing Subscriber configuration
//!
//! ## Caution
//! - Tracing is poorly documented and methods poorly named.  One can easily use, e.g., `::fmt()` instead of `::fmt` and be greeted with cryptic or even misdirecting errors.
//!   - I have no solution for this.  *Just be careful!*  It is very easy to lose a lot of time chasing one's tail, on seemingly trivial configuration.

use tracing::level_filters::LevelFilter;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_error::ErrorLayer;
use tracing_subscriber::{fmt::format::FmtSpan, prelude::*};

use crate::Result;

/// (Convenience function.) Generates a tracing_subcsriber and sets it as global default, while returning a writer guard.
///
/// # Caveat
///   - Side effect. (sets global default tracing subscriber)
///
/// # Use:
/// ```text
/// fn main() -> Result<()> {
///     let _tracing_writer_worker_guard = generate_tracing_subscriber()?;
///    // ...
///    Ok(())
/// }
/// ```
pub fn active_global_default_tracing_subscriber() -> Result<WorkerGuard> {
        let envfilter_layer = tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy();

        let error_layer = ErrorLayer::default().with_filter(LevelFilter::TRACE);

        let (non_blocking_writer, trace_writer_guard) = tracing_appender::non_blocking(std::io::stderr());
        let fmt_layer = tracing_subscriber::fmt::Layer::default()
                // .compact()
                // .pretty()
                // .with_timer(<timer>)
                .with_target(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_file(true)
                .with_line_number(true)
                .with_span_events(FmtSpan::NONE)
                .with_writer(non_blocking_writer)
                .with_filter(envfilter_layer);

        let subscriber = tracing_subscriber::Registry::default()
                .with(error_layer)
                .with(fmt_layer);

        tracing::subscriber::set_global_default(subscriber)?;
        Ok(trace_writer_guard)
}
