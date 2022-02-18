use serde_json::{Map, Value};
use crate::imp::convert::convert_value_str::convert_value_str;
use crate::imp::convert::create_ref_ev_json_array::create_rev_ev_json_array;
use crate::NlResult;
use crate::imp::structs::weak_value::WeakValue;
use crate::imp::structs::cond_value::CondValue;
use crate::imp::structs::param_name_map::ParamNameMap;

pub(crate) fn convert_weak(array : Vec<Value>, filename : &str, names : &ParamNameMap) -> NlResult<Vec<Value>>{
    let mut r : Vec<Value> = Vec::with_capacity(array.len() + 1);
    r.push(Value::String("Cil".to_string()));
    for (index, item) in array.into_iter().enumerate(){
        match item{
            Value::Array(v) =>{
                let v = convert_inner_v(v, filename, index, names)?;
                let mut map : Map<String, Value> = Map::with_capacity(1);
                map.insert(names.get("seq").to_string(), Value::Array(v));
                r.push(Value::Object(map));
            },
            _ =>{
                Err(format!("{}: v has a non-array item", filename))?;
            }
        }
    }
    Ok(r)
}

fn convert_inner_v(array : Vec<Value>, filename : &str, index : usize, names : &ParamNameMap) -> NlResult<Vec<Value>>{
    let mut r : Vec<Value> = Vec::with_capacity(array.len() + 1);
    r.push(Value::String("Cil".to_string()));
    for item in array{
        match item{
            Value::Object(map) =>{
                let map= convert_weak_obj(map, filename, names)?;
                r.push(Value::Object(map));
            },
            _ =>{
                Err(format!("{}:index {} has a non-object", filename, index))?
            }
        }
    }
    Ok(r)
}

fn convert_weak_obj(mut map : Map<String, Value>, filename : &str, names : &ParamNameMap) -> NlResult<Map<String, Value>>{
    let vp = names.get("v");
    let v = map.remove("v");
    match v{
        Some(Value::String(s)) =>{
            let value_str = convert_value_str(&s, filename)?;
            let wv = WeakValue::from(value_str, filename)?;
            map.insert(vp.to_string(), wv.to_json());
        },
        Some(v) => Err(format!("{}: v must be String {}", filename, v))?,
        None =>{}
    }
    let cp = names.get("c");
    let c = map.remove("c");
    match c{
        Some(Value::String(s)) =>{
            let value_str = convert_value_str(&s, filename)?;
            let cv = CondValue::from(value_str, filename)?;
            map.insert(cp.to_string(), cv.to_json());
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
    let action = map.remove("action");
    match action{
        Some(Value::String(s)) =>{
            let array = create_rev_ev_json_array(vec![s]);
            map.insert("action".to_string(), Value::Array(array));
        },
        Some(v) => Err(format!("{}: bonus must be String {}", filename, v))?,
        None =>{}
    }
    Ok(map)
}