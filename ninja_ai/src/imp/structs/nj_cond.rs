use crate::imp::structs::dstring::{DString, Visib};

pub struct NjCond{
    vis : Option<DString>,
    ev_id : u32,
}