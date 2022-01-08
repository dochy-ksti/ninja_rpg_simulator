use crate::imp::structs::value_str::ValueStr;
use crate::NlResult;
use serde_json::{Value, Map};

pub(crate) enum CondValue{
    Normal(String),
    ///Event ID is in the right String
    Black(String, String),
    ///Event ID is in the right String
    Dark(String, String)
}

impl CondValue{
    pub(crate) fn from(vs : ValueStr, filename : &str) -> NlResult<CondValue>{
        let (kind, first, second) = vs.deconstruct();
        if kind.is_none(){
            if second.is_some() {
                return Err(format!("{}: 'c' 'b:' and 'd:' can take two separate values {}",
                            filename, ValueStr::new(kind, first, second).to_string()))?
            } else {
                return Ok(CondValue::Normal(first))
            }
        }
        let kind = kind.unwrap() as char;
        if second.is_none() {
            return Err(format!("{}: 'c' 'b:' and 'd:' must take two separate values {}",
                        filename, ValueStr::new(Some(kind as u8), first, second).to_string()))?;
        }
        let second = second.unwrap();
        match kind{
            'b' => Ok(CondValue::Black(first, second)),
            'd' => Ok(CondValue::Dark(first, second)),
            _ => Err(format!("{}: kind {} is not valid", filename, kind))?,
        }
    }
    pub(crate) fn to_json(&self) -> Value{
        let item = match self{
            CondValue::Normal(id) =>{
                cond_item(None, None, id.to_string())
            },
            CondValue::Dark(s, id) =>{
                cond_item(Some("d"),Some(s), id.to_string())
            },
            CondValue::Black(s, id) =>{
                cond_item(Some("b"),Some(s), id.to_string())
            }
        };

        Value::Array(vec![
            Value::String("Cil".to_string()),
            Value::Object(item),
        ])
    }


}

fn cond_item(kind : Option<&str>, txt : Option<&str>, ev_id : String) -> Map<String, Value>{
    let mut map : Map<String, Value> = Map::new();
    if let Some(kind) = kind {
        map.insert("kind".to_string(), Value::String(kind.to_string()));
    }
    if let Some(txt) = txt {
        map.insert("txt?".to_string(), Value::String(txt.to_string()));
    }
    map.insert("Ref".to_string(), Value::Object(cond_ref(ev_id)));
    map
}

fn cond_ref(ev_id : String) -> Map<String, Value>{
    let mut map : Map<String, Value> = Map::new();
    map.insert("ev".to_string(), Value::String(ev_id));
    map
}
