use crate::imp::structs::skill_id::SkillID;

pub enum TrainSkill{
    Add(AddSkillPoint),
}

pub struct AddSkillPoint{
    id : SkillID,
    point : i32,
}