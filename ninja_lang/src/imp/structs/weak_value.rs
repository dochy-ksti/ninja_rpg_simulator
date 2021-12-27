use crate::imp::structs::value_str::ValueStr;
use crate::NlResult;

pub(crate) struct WeakValue{
    pub(crate) kind : u8,
    pub(crate) text : Option<String>,
    pub(crate) battle_desc : Option<String>,
}

enum Wt{ Str(u8), Val(u8) }
impl WeakValue{
    pub fn from(vs : ValueStr) -> NlResult<WeakValue>{
        match &vs.kind{
            Some(&kind) =>{
                match char::from(kind){
                    'a' =>{ Wt::Val(kind) }
                    'b' =>
                }
            }
        }
    }
}