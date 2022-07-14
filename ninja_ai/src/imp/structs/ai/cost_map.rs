use std::iter;
use crate::imp::structs::ai::cost_map_item::{CostItem, CostMapItem, State};
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

    pub(crate) fn is_reached(&mut self, id : EventID) -> bool{
        self.vec[id.num() as usize].state() == State::Reached
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
                              iteration : u32,
                              skill_map: &SkillMap,
                              trainings : &mut TrainingCollection) -> bool{
        if let Some(from_id) = from_id{
            debug_assert!(current_id != from_id);
            let state = self.get(from_id).state();
            match state{
                State::Unreachable | State::NoData => unreachable!(),
                State::Reached =>{

                }
            }

        }
    }

    fn first_move(&mut self, trainings : &mut TrainingCollection){
        let cost = skill_map.get_cost(val);
        if cost.less_than(self.get(current_id).total_cost()){
            trainings.push()
        }
    }

    fn apply_move_first(&mut self, current_id : EventID, skill_id : SkillID,val : u32, iteration : u32){
        let current = self.get_mut(current_id);
        *current = CostMapItem::new(Some(CostItem::first(total_cost, iteration, skill_id, val)))
    }

    fn apply_move(&mut self, current_id : EventID, from_id: EventID, total_cost : u32, skill_id : SkillID, val : u32, iteration : u32, skill_map : &SkillMap){
        let (current, from) = get_two(&mut self.vec, current_id.num() as usize, from_id.num() as usize);
        match from.state(){
            State::Reached => apply_move_from_reached(current, skill_id, val, iteration),
            _ => unimplemented!(),
        }
    }
}



fn get_two<T>(vec : &mut Vec<T>, i1 : usize, i2 : usize) -> (&mut T, &mut T){
    let min = i1.min(i2);
    let max = i1.max(i2);
    let (_left, right) = vec.split_at_mut(min);
    let (left2, right2) = right.split_at_mut(max - min);
    let left = left2.get_mut(0).unwrap();
    let right = right2.get_mut(0).unwrap();
    if i1 < i2{ (left, right) }
    else{ (right, left) }
}