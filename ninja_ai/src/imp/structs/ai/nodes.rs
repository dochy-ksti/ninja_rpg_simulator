use std::collections::HashMap;
use crate::imp::structs::ai::node::Node;
use crate::imp::structs::event_id::EventID;

pub struct Nodes{
    map : HashMap<EventID, Node>
}

impl Nodes{
    pub fn new(map : HashMap<EventID,Node>) -> Nodes{ Nodes{ map } }
}