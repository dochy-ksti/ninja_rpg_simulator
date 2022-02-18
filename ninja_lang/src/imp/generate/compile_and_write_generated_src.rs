use std::path::Path;
use docchi::core::structs::RootObject;
use crate::{NlResult, compile, write_generated_src_files};
use docchi::intf::generate_interface;

/// returns generated src
pub fn compile_and_write_generated_src<P1 : AsRef<Path>, P2 : AsRef<Path>, P3 : AsRef<Path>>(
    src_dir : P1,
    target_dir : P2,
    generated_src_dir : P3) -> NlResult<(RootObject, String)>{

    let root = compile(src_dir, target_dir)?;
    let src = generate_interface(&root).to_string();
    write_generated_src_files(generated_src_dir, &src)?;
    Ok((root, src))
}
