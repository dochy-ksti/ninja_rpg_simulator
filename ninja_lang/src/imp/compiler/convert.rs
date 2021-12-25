use crate::NlResult;

pub(crate) fn convert(s : &str) -> NlResult<String>{
    let hoge : serde_json::map::Map<K, V> = json5::from_str(s)?;
}