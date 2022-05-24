use crate::imp::structs::ai::required_skills::RequiredSkills;

pub(crate) struct SkillMapItem{
    repeatable : bool,
    increase : u32,
    required_skills : RequiredSkills,
    distance : usize,
}