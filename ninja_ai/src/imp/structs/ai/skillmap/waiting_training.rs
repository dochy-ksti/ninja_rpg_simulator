use std::cmp::Ordering;
use crate::imp::structs::ai::skillmap::slope::Slope;
use crate::imp::structs::ai::skillmap::training::Training;

/// require 順に並べて、requireが同値ならslope順に並べる。BinaryHeapで使うので逆順
#[derive(Debug)]
pub(crate) struct WaitingTraining{
    training : Training,
    slope : Slope,
}

impl WaitingTraining {
    pub(crate) fn new(training: Training, slope : Slope) -> WaitingTraining {
        WaitingTraining{ training, slope }
    }
}

impl PartialEq for WaitingTraining{
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for WaitingTraining{}

impl PartialOrd<Self> for WaitingTraining {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for WaitingTraining{
    fn cmp(&self, other: &Self) -> Ordering {
        //requiredは小さい順、 slopeは大きい順に出てきてほしい
        let ord = other.training.required().cmp(&self.training.required())
        if ord != Ordering::Equal{ return ord; }
        self.slope.cmp(&other.slope)
    }
}
