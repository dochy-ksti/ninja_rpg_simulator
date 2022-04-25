use crate::imp::structs::event_id::EventID;

pub struct ChainAnd{
    vec : Vec<EventID>,
}

pub struct ChainOr{
    vec : Vec<ChainAnd>
}

pub struct NjChain{
    vec : Vec<ChainOr>
}