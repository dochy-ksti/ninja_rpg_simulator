use crate::imp::structs::nj_chain::NjChain;
use crate::imp::structs::nj_wall::NjWalls;

pub struct NjEvent{
    id : String,
    walls : NjWalls,
    chain : NjChain,
}

impl NjEvent{
    pub fn id(&self) -> &str{ &self.id }
}