use crate::imp::structs::dstring::{DString, Visib};
use crate::imp::structs::event_id::EventID;

pub struct NjCond{
    vis : Option<DString>,
    ev_id : EventID,
}