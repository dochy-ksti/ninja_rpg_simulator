use serde_json::{Map, Value};
use crate::NlResult;
use crate::imp::compiler::convert_value_str::convert_value_str;

pub(crate) fn convert_weak(array : Vec<Value>, filename : &str) -> NlResult<Vec<Value>>{
    let mut r : Vec<Value> = Vec::with_capacity(array.len() + 1);
    r.push(Value::String("Cil".to_string()));
    for (index, item) in array.into_iter().enumerate(){
        match item{
            Value::Array(v) =>{
                let v = convert_inner_v(v, filename, index)?;
                let mut map : Map<String, Value> = Map::with_capacity(1);
                map.insert("seq".to_string(), Value::Array(v));
                r.push(Value::Object(map));
            },
            _ =>{
                Err(format!("{}: v has a non-array item", filename))?;
            }
        }
    }
    Ok(r)
}

fn convert_inner_v(array : Vec<Value>, filename : &str, index : usize) -> NlResult<Vec<Value>>{
    let mut r : Vec<Value> = Vec::with_capacity(array.len() + 1);
    r.push(Value::String("Cil".to_string()));
    for item in array{
        match item{
            Value::Object(map) =>{
                let map= convert_weak_obj(map, filename)?;
                r.push(Value::Object(map));
            },
            _ =>{
                Err(format!("{}:index {} has a non-object", filename, index))
            }
        }
    }
    Ok(r)
}

fn convert_weak_obj(mut map : Map<String, Value>, filename : &str) -> NlResult<Map<String, Value>>{
    let hoge = match map.remove("v"){
        Some(Value::String(s)) =>{
            let value_str = convert_value_str(&s, filename)?;
        }
    };
    unimplemented!()
}