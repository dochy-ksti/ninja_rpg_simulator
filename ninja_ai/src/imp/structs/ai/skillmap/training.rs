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
    pub(crate) fn new(repeatable : bool, increase : u32, required : u32, distance : u32, event_id : EventID) -> Training{
        //increaseが0のtrainingは存在してはいけない
        assert_ne!(increase, 0);
        //distanceは1が最小で、何もなくても実行には1かかると考える。実際そうだし
        assert_ne!(distance, 0);

        Training{ repeatable, increase, required, distance, event_id }
    }
    ///このトレーニングで上がるスキルポイントが、このトレーニングを行うまでにいくつ必要か
    pub(crate) fn required(&self) -> u32{
        self.required
    }
    pub(crate) fn repeatable(&self) -> bool{
        self.repeatable
    }
    /// このトレーニングを行ったときのスキルポイントの上がり幅
    /// 1より大きい
    pub(crate) fn increase(&self) -> u32{
        self.increase
    }
    /// このトレーニングまでに必要なイベント数+このトレーニングで上がるスキルポイント以外のスキルポイントを必要量まで上げるために必要な概算の手数
    /// 1より大きい
    pub(crate) fn distance(&self) -> u32{
        self.distance
    }
    pub(crate) fn event_id(&self) -> EventID{
        self.event_id
    }

    pub(crate) fn calc_slope(&self, avg_increase: u32) -> Slope {
        if self.repeatable {
            if self.distance <= 1 { Slope::new(self.increase, 1) }
            else {
                Slope::new(avg_increase, self.distance + (avg_increase / self.increase))
            }
        } else { Slope::new(self.increase, self.distance) }
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
