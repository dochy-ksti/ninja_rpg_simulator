use crate::NlResult;
use serde_json::Value;

pub(crate) fn convert(s : &str) -> NlResult<String>{
    let value : Value = json5::from_str(s)?;
    Ok(value.to_string())
}