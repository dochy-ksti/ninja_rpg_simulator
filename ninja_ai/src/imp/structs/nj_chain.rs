use crate::imp::structs::ev_id::EvID;

pub struct ChainAnd{
    vec : Vec<EvID>,
}

pub struct ChainOr{
    vec : Vec<ChainAnd>
}

pub struct NjChain{
    vec : Vec<ChainOr>
}