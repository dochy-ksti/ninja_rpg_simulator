use std::cmp::Ordering;
use std::collections::BinaryHeap;
use crate::imp::structs::ai::skillmap::slope::Slope;
use crate::imp::structs::event_id::EventID;

#[derive(Debug)]
pub(crate) struct Training{
    repeatable : bool,
    increase : u32,
    required : u32,
    distance : u32,
    event_id : EventID,
}


impl Training{
    pub(crate) fn required(&self) -> u32{
        self.required
    }
    pub(crate) fn repeatable(&self) -> bool{
        self.repeatable
    }
    pub(crate) fn increase(&self) -> u32{
        self.increase
    }
    pub(crate) fn distance(&self) -> u32{
        self.distance
    }
    pub(crate) fn event_id(&self) -> EventID{
        self.event_id
    }
    pub(crate) fn slope(&self) -> Option<Slope>{
        if self.repeatable{
            let distance = u32::max(self.distance, 1);
            if distance == 1{ Some(Slope::new(self.increase, 1)) }
            else{ Some(Slope::new(self.increase ))}
        } else {
            if self.distance == 0 { None } else {
                Some(Slope::new(self.increase, self.distance))
            }
        }
    }

}

impl PartialOrd<Self> for Training {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Training{
    ///BinaryHeapから小さい順に出てほしいので逆にする
    fn cmp(&self, other: &Self) -> Ordering {
        other.required.cmp(&self.required)
    }
}

impl PartialEq for Training{
    fn eq(&self, other: &Self) -> bool {
        self.event_id == other.event_id
    }
}

impl Eq for Training{}
