use crate::imp::structs::dstring::{DString};
use crate::imp::structs::event_id::EventID;

pub struct Cond {
    vis : Option<DString>,
    ev_id : EventID,
}