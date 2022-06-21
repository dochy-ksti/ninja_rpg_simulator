use crate::imp::structs::skill_id::SkillID;

pub enum TrainSkill{
    Add(AddSkillPoint),
}

#[derive(Debug, Copy, Clone)]
pub struct AddSkillPoint{
    id : SkillID,
    point : u32,
}

impl AddSkillPoint{
    pub fn id(&self) -> SkillID{ self.id }
    pub fn point(&self) -> u32{ self.point }
}