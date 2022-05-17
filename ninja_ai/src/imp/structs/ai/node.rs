use crate::imp::structs::ai::line::Line;
use crate::imp::structs::event_id::EventID;

pub(crate) struct Node{
    id : EventID,
    lines : Vec<Line>,
}