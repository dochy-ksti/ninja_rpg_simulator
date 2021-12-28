use std::intrinsics::unreachable;
use crate::imp::structs::value_str::ValueStr;
use crate::NlResult;

pub(crate) fn convert_value_str(s : &str, filename : &str) -> NlResult<ValueStr>{
    let mut vec : Vec<String> = s.split(':').map(|s| s.to_string()).collect();
    match vec.len(){
        0 => unreachable!(),
        1 => Ok(ValueStr::new(None, vec.remove(0), None)),
        2 =>{
            let first = vec.remove(1);
            let mut kind = vec.remove(0);
            if kind.len() != 1{ Err(format!("{}: kind must be a single byte char, {}:{}", filename, kind, first))? }
            Ok(ValueStr::new(Some(kind.remove(0) as u8), first, None))
        },
        3 =>{
            let second = vec.remove(2);
            let first = vec.remove(1);
            let mut kind = vec.remove(0);
            if kind.len() != 1{ Err(format!("{}: kind must be a single byte char, {}:{}:{} ", filename, kind, first, second))? }
            Ok(ValueStr::new(Some(kind.remove(0) as u8), first, Some(second)))
        },
        _ =>{ Err(format!("{}: Not a valid ValueStr, {:?}", filename, vec))? }
    }
}