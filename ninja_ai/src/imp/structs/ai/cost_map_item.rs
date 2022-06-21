use crate::imp::structs::ai::required_skills::RequiredSkills;

pub(crate) struct CostMapItem{
    item : Option<CostItem>,
}

pub(crate) struct CostItem{
    distance : u32,
    skills : RequiredSkills,
}

impl CostMapItem{
    pub(crate) fn no_data() -> CostMapItem{ CostMapItem{ item : None } }
    pub(crate) fn reached() -> CostMapItem{ CostMapItem{ item : Some(CostItem::empty()) } }
    pub(crate) fn accessible(&self) -> bool{ self.item.is_some() }
}

impl CostItem{
    pub(crate) fn new(distance : u32) -> CostItem{
        CostItem{ distance, skills : RequiredSkills::new() }
    }

    pub(crate) fn empty() -> CostItem{
        CostItem{ distance : 0, skills : RequiredSkills::empty() }
    }
}