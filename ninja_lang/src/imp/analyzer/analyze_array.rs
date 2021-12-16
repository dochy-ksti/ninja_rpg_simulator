use docchi_json5::JVal;
use crate::imp::analyzer::rs_val::RsVal;
use crate::error::NpResult;

pub(crate) fn analyze_array(v : JVal) -> NpResult<RsVal>{
    match v{
        JVal::Array(array, _span) =>{ Err("")? },
        JVal::String(s, _span) => Ok(RsVal::Str(s)),
        _ => Err(format!("not an array {}", v.line_str()))?,
    }
}

pub(crate) fn analyze_array_item(v : JVal) -> NpResult