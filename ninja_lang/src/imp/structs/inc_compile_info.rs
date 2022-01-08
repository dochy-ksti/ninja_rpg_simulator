
use serde::{Serialize, Deserialize};
use std::time::{SystemTime};
use std::collections::HashMap;
use std::ffi::{OsStr};
use crate::NlResult;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct IncCompileInfo{
    map : HashMap<String, u64>
}


fn to_millis(time : SystemTime) -> NlResult<u64>{
    let duration = time.duration_since(SystemTime::UNIX_EPOCH)?;
    Ok(duration.as_millis() as u64)
}

impl IncCompileInfo{
    pub(crate) fn new() -> IncCompileInfo{
        IncCompileInfo{ map : HashMap::new() }
    }

    /// 失敗するわけ無いと思うけど一応Result
    pub(crate) fn add(&mut self, filename : &OsStr, modified_time : SystemTime) -> NlResult<()>{
        self.map.insert(filename.to_string_lossy().to_string(), to_millis(modified_time)?);
        Ok(())
    }

    pub(crate) fn contains(&self, filename : &OsStr, modified_time : SystemTime) -> NlResult<bool>{
        if let Some(millis) = self.map.get(filename.to_string_lossy().as_ref()){
            Ok(*millis == to_millis(modified_time)?)
        } else{
            Ok(false)
        }
    }
}