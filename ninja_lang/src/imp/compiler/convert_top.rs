use crate::NlResult;
use serde_json::{Map, Value};
use crate::imp::compiler::convert_weak::convert_weak;
use crate::imp::compiler::convert_chain::convert_chain;

pub(crate) fn convert(s : &str, filename : &str) -> NlResult<Value>{
    let value : Value = json5::from_str(s)?;
    match value{
        Value::Object(map) =>{
            return convert_top(map, filename);
        },
        _ =>{
            Err(format!("file:{} is not an object", filename))?
        }
    }

}

fn convert_top(mut map : Map<String,Value>, filename : &str) -> NlResult<Value>{
    map.insert("ID".to_string(), Value::String(filename.to_string()));
    let opt = match map.remove("v"){
        None =>{ None },
        Some(Value::Array(v)) =>{
            Some(convert_weak(v, filename)?)
        },
        _ =>{ Err(format!("{}: v must be an array", filename))? },
    };
    if let Some(c) = opt{
        map.insert("v".to_string(), Value::Array(c));
    }
    let opt = match map.remove("chain"){
        None =>{ None },
        Some(Value::Object(map)) =>{
            Some(convert_chain(map, filename)?)
        },
        _ =>{ Err(format!("{}: chain must be an object", filename))? },
    };
    if let Some(chain) = opt{
        map.insert("chain".to_string(), Value::Array(chain));
    }
    Ok(Value::Object(map))
}
