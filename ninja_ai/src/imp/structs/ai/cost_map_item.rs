use crate::imp::structs::ai::required_skills::RequiredSkills;

pub(crate) struct CostMapItem{
    item : Option<CostItem>,
}

pub(crate) struct CostItem{
    distance : u32,
    skills : RequiredSkills,
}

impl CostItem{
    pub(crate) fn new(distance : u32) -> CostItem{
        CostItem{ distance, skills : RequiredSkills::new() }
    }
}