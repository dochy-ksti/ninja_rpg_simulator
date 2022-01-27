#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]

//pub use imp::compiler::convert::compile;

pub use crate::error::NlError;
pub type NlResult<T> = Result<T, NlError>;

pub mod error;
mod imp;
#[cfg(test)]
mod testing;



