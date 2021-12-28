use crate::imp::structs::value_str::ValueStr;
use crate::NlResult;
use serde_json::{Value, Map};

pub(crate) enum WeakValue{
    Any(String),
    Black(String),
    Dark(String)
}

impl WeakValue{
    pub(crate) fn from(vs : ValueStr, filename : &str) -> NlResult<WeakValue>{
        if vs.kind().is_none(){
            Err(format!("{}: 'v' needs 'kind' {}", filename, vs.to_string()))?
        }
        if vs.second().is_some(){
            Err(format!("{}: 'v' can't have 'second' {}",filename, vs.to_string()))?
        }
        let (kind, first, _second) = vs.deconstruct();
        let kind = kind.unwrap() as char;
        match kind{
            'a' => Ok(WeakValue::Any(first)),
            'b' => Ok(WeakValue::Black(first)),
            'd' => Ok(WeakValue::Dark(first)),
            _ => Err(format!("{}: kind {} is not valid", filename, kind))?,
        }
    }
    pub(crate) fn to_json(&self) -> Value{

        let item = match self{
            WeakValue::Any(s) => weak_item("a", s),
            WeakValue::Dark(s) => weak_item("d",s),
            WeakValue::Black(s) => weak_item("b",s),
        };

        Value::Array(vec![
            Value::String("Cil".to_string()),
            Value::Object(item),
        ])
    }


}

fn weak_item(kind : &str, txt : &str) -> Map<String, Value>{
    let mut map : Map<String, Value> = Map::new();
    map.insert("kind".to_string(), Value::String(kind.to_string()));
    map.insert("txt".to_string(), Value::String(txt.to_string()));
    map
}
