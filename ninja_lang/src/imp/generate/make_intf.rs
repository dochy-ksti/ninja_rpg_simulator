use std::path::Path;
use docchi::core::structs::RootObject;
use crate::{NlResult};
use crate::imp::generate::compile_and_write_generated_src::compile_and_write_generated_src;

pub fn make_intf<T, P1 : AsRef<Path>, P2 : AsRef<Path>, P3 : AsRef<Path>, F : Fn(RootObject) -> T>(
    src_dir : P1,
    target_dir : P2,
    generated_src_dir : P3,
    intf_constructor : F,
    intf_constructor_src : &str) -> NlResult<T>{

    let (root, src) = compile_and_write_generated_src(src_dir, target_dir, generated_src_dir)?;
    if src != intf_constructor_src{
        Err("incompatible constructor src")?;
    }
    Ok(intf_constructor(root))
}
