#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct NodeID(u32);

impl NodeID{
    pub fn new(n : u32) -> NodeID{ NodeID(n) }
    pub fn num(&self) -> u32{ self.0 }
}