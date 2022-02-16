use std::collections::HashMap;
use crate::NlResult;
use serde_json::{Map, Value};
use crate::imp::convert::convert_chain::convert_chain;
use crate::imp::convert::convert_weak::convert_weak;
use crate::imp::structs::param_name_map::ParamNameMap;

pub(crate) fn convert(s : &str, filename : &str, names : &ParamNameMap) -> NlResult<Value>{
    let value : Value = json5::from_str(s)?;
    match value{
        Value::Object(map) =>{
            convert_top(map, filename, names)
        },
        _ =>{
            Err(format!("file:{} is not an object", filename))?
        }
    }

}

fn convert_top(mut map : Map<String,Value>, filename : &str, names : &ParamNameMap) -> NlResult<Value>{
    let vp = names.get("value");
    let opt = match map.remove(vp){
        None =>{ None },
        Some(Value::Array(v)) =>{
            Some(convert_weak(v, filename, names)?)
        },
        _ =>{ Err(format!("{}: {} must be an array", vp, filename))? },
    };
    if let Some(c) = opt{
        map.insert(vp.to_string(), Value::Array(c));
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
