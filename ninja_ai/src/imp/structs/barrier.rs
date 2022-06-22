use crate::imp::structs::skill_id::SkillID;

pub struct Barrier {
    skill : SkillID,
    val : u32,
}

impl Barrier{
    pub fn id(&self) -> SkillID{ self.skill }
    pub fn val(&self) -> u32{ self.val }
}