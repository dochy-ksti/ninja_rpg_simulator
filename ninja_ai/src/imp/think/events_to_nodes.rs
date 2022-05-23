use std::borrow::Borrow;
use std::collections::HashMap;
use crate::imp::structs::ai::node::Node;
use crate::imp::structs::ai::nodes::Nodes;
use crate::imp::structs::event::Event;
use crate::imp::structs::event_id::EventID;

pub fn events_to_nodes(events : &[Event]) -> Nodes{
    let mut r : HashMap<EventID, Node> = HashMap::with_capacity(events.len());
    for e in events{

    }
    unimplemented!()
}

fn event_to_node(ev : &Event) -> Node{
    ev.id();

    unimplemented!()
}