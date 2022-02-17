#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]

pub use crate::error::NlError;

pub use crate::ch_def_specifications::ch_def_specifications;
pub use crate::cv_def_specifications::cv_def_specifications;
pub use crate::ev_def_specifications::ev_def_specifications;
pub use crate::imp::compiler::compile::compile;

pub mod error;
mod imp;
#[cfg(test)]#[allow(dead_code)]
mod testing;

pub(crate) mod ev_def_specifications;
pub(crate) mod ch_def_specifications;
pub(crate) mod cv_def_specifications;
pub(crate) mod generate;

pub type NlResult<T> = Result<T, NlError>;
