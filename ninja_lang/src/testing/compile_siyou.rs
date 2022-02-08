use crate::NlResult;
use crate::compile;

#[test]
fn compile_siyou() -> NlResult<()>{

    let _hoge = compile("src/json/siyou/event", "src/testing/siyou_compiled/event")?;
    //eprintln!("{}", to_string_pretty(&hoge)?);
    Ok(())
}