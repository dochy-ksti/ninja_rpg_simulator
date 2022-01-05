use serde_json::{Value, Map};

pub(crate) fn create_rev_ev_json_array(ev_ids : Vec<String>) -> Vec<Value>{
    let mut vec : Vec<Value> = Vec::with_capacity(ev_ids.len());
    vec.push(Value::String("Cil".to_string()));
    for ev_id in ev_ids{
        vec.push(Value::Object(create_ref_ev_json_obj(ev_id)));
    }
    vec
}

fn create_ref_ev_json_obj(ev_id : String) -> Map<String, Value>{
    let mut map = Map::new();
    let mut obj = Map::new();
    obj.insert("ev".to_string(), Value::String(ev_id));
    map.insert("Ref".to_string(), Value::Object(obj));
    map
}