use crate::{NlResult, write_generated_src_files};
use crate::compile;
use docchi::intf::generate_interface;

#[test]
fn compile_siyou() -> NlResult<()>{

    let root = compile("src/json/siyou", "src/testing/siyou_compiled")?;
    let generated =generate_interface(&root).to_string();
    write_generated_src_files("src/generated_src", &generated)?;

    Ok(())
}