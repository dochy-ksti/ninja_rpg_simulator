#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]

//pub use imp::compiler::convert::compile;
pub(crate) mod ev_def_specifications;
pub(crate) mod ch_def_specifications;
pub use crate::error::NlError;
pub type NlResult<T> = Result<T, NlError>;
pub use crate::ev_def_specifications::ev_def_specifications;
pub use crate::ch_def_specifications::ch_def_specifications;
pub use crate::imp::compiler::compile::compile;

pub mod error;
mod imp;
#[cfg(test)]#[allow(dead_code)]
mod testing;





