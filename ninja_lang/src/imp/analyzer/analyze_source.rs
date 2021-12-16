use crate::error::NpResult;

pub(crate) fn analyze_source(s : &str) -> NpResult<()>{
    let hoge = docchi_json5::from_str(s)?;
    Ok(())
}