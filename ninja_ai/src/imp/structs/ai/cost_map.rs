use std::iter;
use crate::imp::structs::ai::cost_map_item::CostMapItem;
use crate::imp::structs::ai::skillmap::skill_map::SkillMap;
use crate::imp::structs::ai::skillmap::training_collection::TrainingCollection;
use crate::imp::structs::event_id::EventID;
use crate::imp::structs::skill_id::SkillID;

pub(crate) struct CostMap{
    vec : Vec<CostMapItem>
}

impl CostMap{
    pub(crate) fn new(num_events : usize) -> CostMap{
        CostMap {
            vec : iter::repeat_with(|| CostMapItem::no_data()).take(num_events).collect()
        }
    }
    pub(crate) fn reached(&mut self, id : EventID){
        self.vec[id.num() as usize] = CostMapItem::reached();
    }

    pub(crate) fn get(&self, id : EventID) -> &CostMapItem{
        self.vec.get(id.num() as usize).unwrap()
    }

    pub(crate) fn get_mut(&mut self, id : EventID) -> &mut CostMapItem{
        self.vec.get_mut(id.num() as usize).unwrap()
    }

    pub(crate) fn reachable(&self, id : EventID) -> bool{
        self.get(id).reachable()
    }

    /// 無限ループを避けるためにフラグを立てる
    pub(crate) fn set_guard(&mut self, id: EventID, iteration : u32) {
        self.get_mut(id).set_guard(iteration);
    }

    pub(crate) fn update_cost(&mut self,
                              skill_id : SkillID,
                              val : u32,
                              current_id : EventID,
                              from_id: Option<EventID>,
                              skill_map: &SkillMap,
                              tcol : &mut TrainingCollection) {
        todo!()
    }
}