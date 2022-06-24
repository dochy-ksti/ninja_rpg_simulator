use crate::imp::structs::ai::required_skills::RequiredSkills;

pub(crate) struct CostMapItem{
    item : Option<CostItem>,
}



pub(crate) struct CostItem{
    distance : u32,
    iteration : u32,
    total_cost : u32,
    skills : RequiredSkills,
}

impl CostMapItem{
    pub(crate) fn no_data() -> CostMapItem{ CostMapItem{ item : None } }
    pub(crate) fn reached() -> CostMapItem{ CostMapItem{ item : Some(CostItem::empty()) } }
    //pub(crate) fn reachable(&self) -> bool{ self.item.is_some() }
    pub(crate) fn item(&self) -> Option<&CostItem>{ self.item.as_ref() }
    pub(crate) fn set_unreachable(&mut self, iteration: u32) {
        self.item = Some(CostItem::unreachable())
    }
}

impl CostItem{
    pub(crate) fn new(distance : u32, iteration : u32) -> CostItem{
        CostItem{ distance, iteration, skills : RequiredSkills::new() }
    }

    pub(crate) fn empty() -> CostItem{
        CostItem{ distance : 0, iteration : 0, total_cost : 0, skills : RequiredSkills::empty() }
    }

    pub(crate) fn unreachable(iteration : u32) -> CostItem{
        CostItem{ distance : u32::MAX, iteration, total_cost : U32::MAX, skills : RequiredSkills::empty() }
    }

    pub(crate) fn distance(&self) -> u32{ self.distance }
    pub(crate) fn iteration(&self) -> u32{ self.iteration }

    pub(crate) fn is_reached(&self) -> bool{ self.skills.is_empty() }
}