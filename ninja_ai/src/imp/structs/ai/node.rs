use crate::imp::structs::ai::line::Line;
use crate::imp::structs::event_id::EventID;

pub struct Node{
    id : EventID,
    lines : Vec<Line>,
}

impl Node{
    pub(crate) fn new(id : EventID, lines : Vec<Line>) -> Node{ Node{ id, lines } }
}

