use ordered_float::NotNan;
use crate::imp::structs::ai::required_skills::RequiredSkills;
use crate::imp::structs::event_id::EventID;

pub(crate) struct SkillMapItem {
    repeatable : bool,
    increase : u32,
    cost : u32,
}

pub(crate) struct Repeatable{
    stacked_distance : u32,
    stacked_skill_point : u32,
    slope : u32,
    event_id : EventID,
}

pub(crate) struct Once{
    stacked_distance : u32,
    stacked_skill_point : u32,
    event_id : EventID,
}