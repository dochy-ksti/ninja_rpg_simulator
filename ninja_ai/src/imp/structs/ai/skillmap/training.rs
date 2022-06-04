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
    pub(crate) fn new(repeatable : bool, increase : u32, required : u32, distance : u32) -> Training{
        //increaseが0のtrainingは存在してはいけない
        assert_ne!(increase, 0);
        //distanceは1が最小で、何もなくても実行には1かかると考える。実際そうだし
        assert_ne!(distance, 0);
    }
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


    pub(crate) fn calc_slope(repeatable : bool, increase : u32, distance : u32, avg_increase: u32) -> Option<Slope> {
        if repeatable {
            if distance == 0 { Some(Slope::new(increase, 1)) }
            else {
                let burden = f32::floor(avg_increase / increase as f32) as u32;
                Some(Slope::new(avg_increase, distance + burden))
            }
        } else {
            if distance == 0 { None } else {
                Some(Slope::new(increase, distance))
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
