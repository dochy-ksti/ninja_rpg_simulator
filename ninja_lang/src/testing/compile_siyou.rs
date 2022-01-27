use serde_json::to_string_pretty;
use crate::NlResult;
use crate::imp::translator::convert_top::convert;

#[test]
fn compile_siyou() -> NlResult<()>{
    let s = std::fs::read_to_string("event仕様.json5")?;
    let hoge = convert(&s, "siyou.json5")?;
    eprintln!("{}", to_string_pretty(&hoge)?);
    Ok(())
}