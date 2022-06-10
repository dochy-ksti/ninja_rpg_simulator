use std::collections::HashMap;
use crate::imp::structs::ai::node::Node;
use crate::imp::structs::ai::nodes::Nodes;
use crate::imp::structs::event::Event;
use crate::imp::structs::event_id::EventID;

pub fn events_to_nodes(_events: &[Event]) -> Nodes{
    let mut _r : HashMap<EventID, Node> = HashMap::with_capacity(_events.len());
    for _e in _events {

    }
    unimplemented!()
}

fn event_to_node(_ev : &Event) -> Node{

    unimplemented!()
}