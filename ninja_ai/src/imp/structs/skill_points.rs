use std::iter::repeat;
use crate::imp::structs::skill_id::SkillID;

pub struct SkillPoints{
    vec : Vec<u32>
}

impl SkillPoints{
    pub fn new() -> SkillPoints{
        SkillPoints{ vec : repeat(0).take(SkillID::len()).collect() }
    }

    pub fn get(&self, id : SkillID) -> u32{
        self.vec[id.num() as usize]
    }

    pub fn set(&mut self, id : SkillID, val : u32){
        self.vec[id.num() as usize] = val;
    }
}