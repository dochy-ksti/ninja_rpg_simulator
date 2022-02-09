use crate::NlResult;
use crate::compile;

#[test]
fn compile_siyou() -> NlResult<()>{

    let _hoge = compile("src/json/siyou", "src/testing/siyou_compiled")?;
    //eprintln!("{}", to_string_pretty(&hoge)?);
    Ok(())
}