#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]

//pub use imp::compiler::convert::compile;
pub(crate) mod ev_def_specifications;

pub use crate::error::NlError;
pub type NlResult<T> = Result<T, NlError>;
pub use crate::ev_def_specifications::ev_def_specifications;

pub mod error;
mod imp;
#[cfg(test)]
mod testing;




