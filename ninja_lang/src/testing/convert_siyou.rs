use crate::NlResult;
use crate::imp::translate_ev::convert_top::convert;


//#[test]
fn convert_siyou() -> NlResult<()>{
    let s = std::fs::read_to_string("event仕様.json5")?;
    let _hoge = convert(&s, "siyou.json5")?;
    //eprintln!("{}", to_string_pretty(&hoge)?);
    Ok(())
}