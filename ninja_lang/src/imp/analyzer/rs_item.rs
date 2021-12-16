use std::collections::HashMap;
use crate::imp::analyzer::rs_val::RsVal;

pub(crate) struct RsItem{
    map : HashMap<String, RsVal>
}