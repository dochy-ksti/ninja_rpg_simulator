#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]

pub use imp::compiler::compile::compile;

pub use crate::error::NlError;

pub mod error;
mod imp;

pub type NlResult<T> = Result<T, NlError>;

