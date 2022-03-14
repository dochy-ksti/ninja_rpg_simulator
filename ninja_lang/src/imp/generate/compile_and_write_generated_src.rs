use std::path::Path;
use docchi::core::structs::RootObject;
use crate::{NlResult, compile, write_generated_src_file};
use docchi::intf::generate_interface;

/// returns Ok when the file is changed
pub fn compile_and_write_generated_src<P1 : AsRef<Path>, P2 : AsRef<Path>, P3 : AsRef<Path>>(
    src_dir : P1,
    target_dir : P2,
    generated_src_file : P3) -> NlResult<Result<(RootObject, String), (RootObject, String)>>{

    let root = compile(src_dir, target_dir)?;
    let src = generate_interface(&root).to_string();
    if write_generated_src_file(generated_src_file, &src)?{
        Ok(Err((root, src)))
    } else {
        Ok(Ok((root, src)))
    }
}
