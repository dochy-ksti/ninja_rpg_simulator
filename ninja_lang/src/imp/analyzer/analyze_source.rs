use crate::error::NpResult;
use docchi_json5::JVal;

pub(crate) fn analyze_source(s : &str) -> NpResult<()>{
    match docchi_json5::from_str(s)?{
        JVal::Array(a, _span){
            let mut vec = vec![]
            for item in a{

            }
        }
    }

    Ok(())
}