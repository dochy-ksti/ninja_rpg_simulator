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

pub(crate) enum State{
    Unreachable, Reached, Normal, NoData
}

impl CostMapItem{
    pub(crate) fn no_data() -> CostMapItem{ CostMapItem{ item : None } }
    pub(crate) fn reached() -> CostMapItem{ CostMapItem{ item : Some(CostItem::empty()) } }
    pub(crate) fn state(&self) -> State{
        if let Some(item) = self.item.as_ref(){
            if item.is_unreachable(){
                State::Unreachable
            } else if item.skills.is_empty(){
                State::Reached
            } else{
                State::Normal
            }
        } else{
            State::NoData
        }
    }
    pub(crate) fn total_cost(&self) -> u32{
        if let Some(item) = self.item{
            item.total_cost
        } else{
            u32::MAX
        }
    }
    pub(crate) fn item(&self) -> Option<&CostItem>{ self.item.as_ref() }
    pub(crate) fn set_guard(&mut self, iteration: u32) {
        if self.item.is_none() {
            self.item = Some(CostItem::unreachable(iteration))
        } else{
            self.item.unwrap().iteration = iteration;
        }
    }

    /// Not Unreachable nor NoData
    pub(crate) fn reachable(&self) -> bool {
        if let Some(item) = self.item{
            if item.is_unreachable(){
                false
            } else{
                true
            }
        } e3lse{
            false
        }
    }
}

impl CostItem{
    pub(crate) fn new(distance : u32, iteration : u32, total_cost : u32, skills : RequiredSkills) -> CostItem{
        CostItem{ distance, iteration, skills, total_cost }
    }

    pub(crate) fn empty() -> CostItem{
        CostItem{ distance : 0, iteration : 0, total_cost : 0, skills : RequiredSkills::empty() }
    }

    pub(crate) fn unreachable(iteration : u32) -> CostItem{
        CostItem{ distance : u32::MAX, iteration, total_cost : U32::MAX, skills : RequiredSkills::empty() }
    }

    pub(crate) fn distance(&self) -> u32{ self.distance }
    pub(crate) fn iteration(&self) -> u32{ self.iteration }
    pub(crate) fn skills(&self) -> &RequiredSkills{ &self.skills }
    pub(crate) fn total_cost(&self) -> u32{ self.total_cost }

    pub(crate) fn is_reached(&self) -> bool{ self.distance == 0 }
    pub(crate) fn is_unreachable(&self) -> bool{ self.distance == u32::MAX }

}