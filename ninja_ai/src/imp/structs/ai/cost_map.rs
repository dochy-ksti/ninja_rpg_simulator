use std::iter;
use crate::imp::structs::ai::cost_map_item::{CostMapItem, State};
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
        if let Some(from_id) = from_id{
            debug_assert!(current_id != from_id);
            let state = self.get(from_id).state();
            match state{
                State::Unreachable | State::NoData => unreachable!(),
                State::Reached =>{
                    let cost = skill_map.get_cost(val);
                    if cost < self.get(current_id).total_cost(){

                    }
                }
            }

        }
    }

    fn apply_move(&mut self, current_id : EventID, from_id: EventID, skill_id : SkillID, val : u32){
        let (current, from) = get_two(&mut self.vec, current_id.num() as usize, from_id.num() as usize);

    }
}

fn get_two<T>(vec : &mut Vec<T>, i1 : usize, i2 : usize) -> (&mut T, &mut T){
    let min = i1.min(i2);
    let max = i1.max(i2);
    let (_left, right) = vec.split_at_mut(min);
    let (left2, right2) = right.split_at_mut(max - min);
    let left = left2.get_mut(0).unwrap();
    let right = right2.get_mut(0).unwrap())
    if i1 < i2{ (left, right) }
    else{ (right, left) }
}