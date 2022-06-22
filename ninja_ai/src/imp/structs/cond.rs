use crate::imp::structs::dstring::{DString};
use crate::imp::structs::event_id::EventID;

pub struct Cond {
    vis : Option<DString>,
    ev_id : EventID,
}

impl Cond{
    pub fn id(&self) -> EventID{ self.ev_id }
    pub fn vis(&self) -> Option<&DString>{ self.vis.as_ref() }
}