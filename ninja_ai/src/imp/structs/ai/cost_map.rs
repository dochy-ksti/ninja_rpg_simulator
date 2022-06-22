use std::iter;
use crate::imp::structs::ai::cost_map_item::CostMapItem;
use crate::imp::structs::event_id::EventID;

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

    pub(crate) fn reachable(&mut self, id : EventID) -> bool{
        self.get(id).reachable()
    }
}