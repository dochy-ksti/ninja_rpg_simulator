use std::intrinsics::unreachable;
use crate::imp::structs::value_str::ValueStr;
use crate::NlResult;

pub(crate) fn convert_value_str(s : &str) -> NlResult<ValueStr>{
    let vec : Vec<String> = s.split(':').map(|s| s.to_string()).collect();
    match vec.len(){
        0 => unreachable!(),
        1 =>
    }
}