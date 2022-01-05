use serde_json::{Map, Value};
use crate::NlResult;
use crate::imp::compiler::convert_value_str::convert_value_str;
use crate::imp::structs::weak_value::WeakValue;
use crate::imp::structs::cond_value::CondValue;
use crate::imp::compiler::create_ref_ev_json_array::create_rev_ev_json_array;

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

    match o {
        Value::Array(v) => {
            let v = convert_o(v, filename, index)?;
            let mut map: Map<String, Value> = Map::with_capacity(1);
            map.insert("seq".to_string(), Value::Array(v));
            r.push(Value::Object(map));
        },
        _ => {
            Err(format!("{}: v has a non-array item", filename))?;
        }
    }

    match a {
        Value::Array(v) => {
            let v = convert_o(v, filename)?;
            let mut map: Map<String, Value> = Map::with_capacity(1);
            map.insert("seq".to_string(), Value::Array(v));
            r.push(Value::Object(map));
        },
        _ => {
            Err(format!("{}: v has a non-array item", filename))?;
        }
    }

    Ok(r)
}

fn convert_o(array : Vec<Value>, filename : &str, index : usize) -> NlResult<Vec<Value>>{
    if array.len() == 0{
        Err(format!("{}:chain:o: array is empty", filename))?
    }
    let mut r : Vec<Value> = Vec::with_capacity(array.len() + 1);
    r.push(Value::String("Cil".to_string()));
    for (index, item) in array.into_iter().enumerate(){
        match item{
            Value::String(id) =>{
                let and = create_rev_ev_json_array(vec![id]);
                let mut map = Map::new();
                map.insert("and".to_string(), Value::Array(and))
                r.push(Value::Object(map));
            },
            _ =>{
                Err(format!("{}:chain:o: index {} is not a string", filename, index))?
            }
        }
    }
    Ok(r)
}

fn convert_weak_obj(mut map : Map<String, Value>, filename : &str) -> NlResult<Map<String, Value>>{
    let v = map.remove("v");
    match v{
        Some(Value::String(s)) =>{
            let value_str = convert_value_str(&s, filename)?;
            let wv = WeakValue::from(value_str, filename)?;
            map.insert("v".to_string(), wv.to_json());
        },
        Some(v) => Err(format!("{}: v must be String {}", filename, v))?,
        None =>{}
    }
    let c = map.remove("c");
    match c{
        Some(Value::String(s)) =>{
            let value_str = convert_value_str(&s, filename)?;
            let cv = CondValue::from(value_str, filename)?;
            map.insert("c".to_string(), cv.to_json());
        },
        Some(v) => Err(format!("{}: c must be String {}", filename, v))?,
        None =>{}
    }
    let run = map.remove("run");
    match run{
        Some(Value::String(s)) =>{
           let array = create_rev_ev_json_array(vec![s]);
            map.insert("run".to_string(), Value::Array(array));
        },
        Some(v) => Err(format!("{}: run must be String {}", filename, v))?,
        None =>{}
    }
    let bonus = map.remove("bonus");
    match bonus{
        Some(Value::String(s)) =>{
            let array = create_rev_ev_json_array(vec![s]);
            map.insert("bonus".to_string(), Value::Array(array));
        },
        Some(v) => Err(format!("{}: bonus must be String {}", filename, v))?,
        None =>{}
    }
    Ok(map)
}