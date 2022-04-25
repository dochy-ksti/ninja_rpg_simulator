#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventID(u32);

impl EventID{
    pub fn new(n : u32) -> EventID{ EventID(n) }
    pub fn num(&self) -> u32{ self.0 }
}