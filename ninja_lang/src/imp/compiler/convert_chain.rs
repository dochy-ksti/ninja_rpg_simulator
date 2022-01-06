use serde_json::{Map, Value};
use crate::NlResult;
use crate::imp::compiler::create_ref_ev_json_array::create_rev_ev_json_array;
use crate::imp::util::json_array_to_string_vec::json_array_to_string_vec;

pub(crate) fn convert_chain(mut obj : Map<String, Value>, filename : &str) -> NlResult<Vec<Value>> {
    let mut r: Vec<Value> = Vec::new();
    r.push(Value::String("Cil".to_string()));
    let o = obj.remove("o");
    let a = obj.remove("a");
    if o.is_some() && a.is_some(){
        Err(format!("{}: chain has o and a", filename))?;
    }
    if obj.is_empty() == false{
        Err(format!("{}: chain can only have o or a", filename))?;
    }
    if o.is_none() && a.is_none(){
        Err(format!("{}: chain doesn't have o or a", filename))?;
    }

    match o {
        Some(Value::Array(v)) => {
            if v.len() == 0{
                Err(format!("{}:chain: o is empty", filename))?;
            }
            let v = convert_o(v, filename)?;
            let mut map: Map<String, Value> = Map::with_capacity(1);
            map.insert("chain".to_string(), Value::Array(v));
            r.push(Value::Object(map));
        },
        Some(_) => {
            Err(format!("{}: v has a non-array item", filename))?;
        },
        None => {},
    }

    match a {
        Some(Value::Array(v)) => {
            if v.len() == 0{
                Err(format!("{}:chain: a is empty", filename))?;
            }
            let v = if v[0].as_array().is_some(){
                convert_and2(v, filename)?
            } else{
                convert_and1(v, filename)?
            };
            let v = convert_o(v, filename)?;
            let mut map: Map<String, Value> = Map::with_capacity(1);
            map.insert("chain".to_string(), Value::Array(v));
            r.push(Value::Object(map));
        },
        Some(_) => {
            Err(format!("{}: v has a non-array item", filename))?;
        },
        None =>{},
    }

    Ok(r)
}

fn convert_o(array : Vec<Value>, filename : &str) -> NlResult<Vec<Value>>{
    let mut r : Vec<Value> = Vec::with_capacity(array.len() + 1);
    r.push(Value::String("Cil".to_string()));
    for (index, item) in array.into_iter().enumerate(){
        match item{
            Value::String(id) =>{
                let and = create_rev_ev_json_array(vec![id]);
                let mut map = Map::new();
                map.insert("and".to_string(), Value::Array(and));
                r.push(Value::Object(map));
            },
            _ =>{
                Err(format!("{}:chain:o: index {} is not a string", filename, index))?
            }
        }
    }
    Ok(r)
}

fn convert_and2(array : Vec<Value>, filename : &str) -> NlResult<Vec<Value>>{
    let mut r : Vec<Value> = Vec::with_capacity(array.len() + 1);
    r.push(Value::String("Cil".to_string()));
    for (index, item) in array.into_iter().enumerate(){
        match item{
            Value::Array(vec) =>{
                if vec.len() == 0{
                    Err(format!("{}:chain:a: index {} is empty", filename, index))?
                }
                let ids = json_array_to_string_vec(vec).ok_or_else(||{
                    format!("{}:chain:a: index {} has non-string", filename, index)
                })?;
                let and = create_rev_ev_json_array(ids);
                let mut map = Map::new();
                map.insert("and".to_string(), Value::Array(and));
                r.push(Value::Object(map));
            },
            _ =>{
                Err(format!("{}:chain:a: index {} is not an array", filename, index))?
            }
        }
    }
    Ok(r)
}

fn convert_and1(array : Vec<Value>, filename : &str) -> NlResult<Vec<Value>> {
    let mut r: Vec<Value> = Vec::with_capacity(array.len() + 1);
    r.push(Value::String("Cil".to_string()));
    let ids = json_array_to_string_vec(array).ok_or_else(|| {
        format!("{}:chain: a has non-string", filename)
    })?;
    let and = create_rev_ev_json_array(ids);
    let mut map = Map::new();
    map.insert("and".to_string(), Value::Array(and));
    r.push(Value::Object(map));
    Ok(r)
}