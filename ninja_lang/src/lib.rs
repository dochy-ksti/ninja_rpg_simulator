#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]

pub use crate::ch_def_specifications::CH_DEF_SPECIFICATIONS;
pub use crate::cv_def_specifications::CV_DEF_SPECIFICATIONS;
pub use crate::error::NlError;
pub use crate::ev_def_specifications::EV_DEF_SPECIFICATIONS;
pub use crate::imp::compiler::compile::compile;
pub use crate::imp::generate::write_generated_src_files::write_generated_src_files;

pub mod error;
mod imp;
#[cfg(test)]#[allow(dead_code)]
mod testing;

pub(crate) mod ev_def_specifications;
pub(crate) mod ch_def_specifications;
pub(crate) mod cv_def_specifications;
pub(crate) mod generate_def_specifications;

pub type NlResult<T> = Result<T, NlError>;
