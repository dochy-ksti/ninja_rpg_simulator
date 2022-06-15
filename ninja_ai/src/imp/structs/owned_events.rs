use bit_vec::BitVec;
use crate::imp::structs::event_id::EventID;

pub struct OwnedEvents{
    vec : BitVec<u64>,
}

impl OwnedEvents{
    pub fn new(num_events : usize) -> OwnedEvents{
        let mut vec : BitVec<u64> = BitVec::default();
        vec.grow(num_events, false);
        OwnedEvents{ vec }
    }

    pub fn get(&self, id : EventID) -> bool{
        self.vec.get(id.num() as usize).unwrap()
    }

    pub fn set(&mut self, id : EventID){
        self.vec.set(id.num() as usize, true);
    }
}