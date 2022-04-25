#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SkillID(u32);

impl SkillID{
    pub fn new(n : u32) -> SkillID{ SkillID(n) }
    pub fn num(&self) -> u32{ self.0 }
}