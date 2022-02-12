use std::fs::read_dir;
use std::path::Path;
use serde_json::{json, to_string_pretty, Value};
use crate::NlResult;


pub(crate) fn calc_cv<'a, I : Iterator<Item=&'a str>>(names : I) -> NlResult<String>{
    let mut json = json!(["MList", [{
        "Ref":{ "ch" : "" },
        "own" : ["MilDef",[{
            "Ref" : { "ev" : "" }
        }]]
    }]]);
    let mut vec = if let Value::Array(vec) = json{ vec } else{ unreachable!() };

    for name in names{
        let item = json!(
            {
                "Ref":{ "ch" : name.to_string() },
            }
        );
        vec.push(item);
    }
    let array = Value::Array(vec);
    Ok(to_string_pretty(&array)?)
}