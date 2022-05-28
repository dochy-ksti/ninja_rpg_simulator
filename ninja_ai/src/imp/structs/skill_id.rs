use std::collections::HashSet;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SkillID(u32);

impl SkillID{
    pub fn new(n : u32) -> SkillID{
        assert!((n as usize) < Self::len());
        SkillID(n)
    }
    pub fn num(&self) -> u32{ self.0 }
    pub const fn len() -> usize{ 2 }
}

pub static SkillNames : Lazy<HashSet<String>> = Lazy::new(||{
    let mut set = HashSet::new();
    set.insert("sword".to_string());
    set.insert("magic".to_string());
    assert_eq!(set.len(), SkillID::len());
    set
});