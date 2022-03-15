use std::path::Path;
use docchi::core::structs::RootObject;
use crate::{NlResult};
use crate::imp::generate::compile_and_write_generated_src::compile_and_write_generated_src;

pub fn make_intf<T, P1 : AsRef<Path>, P2 : AsRef<Path>, P3 : AsRef<Path>, F : Fn(RootObject) -> T>(
    src_dir : P1,
    target_dir : P2,
    generated_src_file : P3,
    intf_constructor : F) -> NlResult<T>{

    if let Err((root, _src)) = compile_and_write_generated_src(src_dir, target_dir, generated_src_file)?{
        return Ok(intf_constructor(root));
    } else {
        Err("intf is old and may be incompatible")?
    }

}
