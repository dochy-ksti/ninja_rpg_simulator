use crate::imp::structs::dstring::{DString, Visib};
use crate::imp::structs::ev_id::EvID;

pub struct NjCond{
    vis : Option<DString>,
    ev_id : EvID,
}