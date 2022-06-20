use crate::imp::structs::event::Event;
use crate::imp::structs::event_id::EventID;

pub struct Events{
    vec : Vec<Event>
}

impl Events{
    pub fn new(vec : Vec<Event>) -> Events{ Events{ vec } }
    pub fn get(&self, id : EventID) -> &Event{ self.vec.get(id.num() as usize).unwrap() }
    pub fn len(&self) -> usize{ self.vec.len() }
}