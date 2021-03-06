#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CharaID(u32);

impl CharaID{
    pub fn new(n : u32) -> CharaID{ CharaID(n) }
    pub fn num(&self) -> u32{ self.0 }
}