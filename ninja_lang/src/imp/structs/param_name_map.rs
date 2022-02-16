use std::collections::HashMap;

pub(crate) struct ParamNameMap{
    map : HashMap<String, String>
}

impl ParamNameMap{
    pub(crate) fn new() -> ParamNameMap{
        ParamNameMap{ map : HashMap::new() }
    }

    pub(crate) fn add(&mut self, key : &str, val : &str){
        self.map.insert(key.to_string(), val.to_string());
    }

    /// Panics when this fails
    pub(crate) fn get(&self, s : &str) -> &str{
        self.map.get(s).unwrap()
    }
}