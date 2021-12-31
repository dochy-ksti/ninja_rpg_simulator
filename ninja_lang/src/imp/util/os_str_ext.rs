// use std::ffi::{OsString, OsStr};
// use crate::NlResult;
//
// pub(crate) trait OsStrExt{
//     fn m_str(&self) -> NlResult<&str>;
// }
//
// impl OsStrExt for OsString{
//     fn m_str(&self) -> NlResult<&str> {
//         Ok(self.to_str().ok_or_else(||"OsString couldn't be converted to Rust String")?)
//     }
// }
//
// impl OsStrExt for &OsStr{
//     fn m_str(&self) -> NlResult<&str> {
//         Ok(self.to_str().ok_or_else(||"OsString couldn't be converted to Rust String")?)
//     }
// }