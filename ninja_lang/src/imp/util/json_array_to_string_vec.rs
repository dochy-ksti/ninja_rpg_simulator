use serde_json::Value;

pub(crate) fn json_array_to_string_vec(array : Vec<Value>) -> Option<Vec<String>>{
    array.into_iter().map(|value| {
        match value{
            Value::String(s) => Some(s),
            _ => None,
        }
    }).collect()
}