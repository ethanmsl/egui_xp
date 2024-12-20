//! Error
//!
//!
//! ## Utility reference
//! For adding backtrace to errors:
//! `#![feature(error_generic_member_access)]`
//! `use std::backtrace;`

use std::io;

use derive_more::{Display, Error, From};
use tracing::{instrument, subscriber::SetGlobalDefaultError};

// use derive_more::{Display, Error, derive::From};
#[derive(Debug, Display, From, Error)]
pub enum ErrKind {
        //
        // `custom` errors
        #[from(ignore)]
        #[display("Unparsable character: {}", source_char)]
        ParseOther { source_char: char },
        #[from(ignore)]
        #[display("Error extracting lines from input: {}", source_input)]
        NoInputLines { source_input: String },
        //
        // `packed` errors
        #[display("io error: {}", source)]
        Io { source: io::Error },
        #[display("Error setting tracing subscriber default: {}", source)]
        TracingSubscriber { source: SetGlobalDefaultError },
        //
        // `other` errors
        #[from(ignore)] // use `make_dyn_error` instead; would conflict with auto-derives
        #[display("Uncategorized Error (dyn error object): {}", source)]
        OtherDynError {
                source: Box<dyn std::error::Error + Send + Sync>,
        },
        #[display(r#"Uncategorized string err: "{}""#, source_string)]
        OtherStringError { source_string: String },
        //
        // // common error types
        // #[from(ignore)]
        // #[display("error parsing char: {}", uninterpretable_char)]
        // CharParse { uninterpretable_char: char },
        // #[display("CLI parsing library error: {}", source)]
        // Clap { source: clap::Error },
        // #[display("env variable error: {}", source)]
        // Env { source: env::VarError },
        // #[display("parse error: {}", source)]
        // ParseInt { source: num::ParseIntError },
}
impl ErrKind {
        #[instrument(skip_all)]
        pub fn make_dyn_error<E>(error: E) -> Self
        where
                E: Into<Box<dyn std::error::Error + Send + Sync>>,
        {
                Self::OtherDynError { source: error.into() }
        }
}

#[derive(Display, Error)]
#[display(
        "error: {:#}",
        // "error: {:#}\n\n\nspantrace capture: {:?}\n\n\nspantrace: {:#}",
        source,
        // spantrace.status(),
        // spantrace,
)]
pub struct ErrorWrapper {
        pub source: ErrKind,
        // pub spantrace: tracing_error::SpanTrace,
        // backtrace: backtrace::Backtrace,
}
// Using custom display as debug so we can get SpanTrace auto printed.
impl std::fmt::Debug for ErrorWrapper {
        #[instrument(skip_all)]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self)
        }
}
impl<T> From<T> for ErrorWrapper
where
        T: Into<ErrKind>,
{
        #[instrument(skip_all)]
        fn from(error: T) -> Self {
                Self {
                        source: error.into(),
                        // spantrace: tracing_error::SpanTrace::capture(),
                        // backtrace: backtrace::Backtrace::capture(),
                }
        }
}

pub trait ToOther {
        fn to_other(self) -> ErrorWrapper;
}
impl<E> ToOther for E
where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
        #[instrument(skip_all)]
        fn to_other(self) -> ErrorWrapper {
                ErrKind::OtherDynError { source: self.into() }.into()
        }
}
