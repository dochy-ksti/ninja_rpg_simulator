use std::cmp::Ordering;
use std::collections::BinaryHeap;
use crate::imp::structs::ai::skillmap::slope::Slope;
use crate::imp::structs::ai::skillmap::training::Training;

pub(crate) struct AvailableTrainings{
    bh : BinaryHeap<AvailableTraining>,
}

impl AvailableTrainings{
    pub(crate) fn new() -> AvailableTrainings{

    }
    pub(crate) fn peek(&self) -> Option<&AvailableTraining>{
        self.bh.peek()
    }
    pub(crate) fn pop(&mut self) -> Option<AvailableTraining>{
        self.bh.pop()
    }
}

#[derive(Debug)]
pub(crate) struct AvailableTraining{
    slope : Slope,
    training : Training,
}

impl PartialEq for AvailableTraining{
    fn eq(&self, other: &Self) -> bool {
        self.training == other.training
    }
}

impl Eq for AvailableTraining{}

impl PartialOrd<Self> for AvailableTraining {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for AvailableTraining{
    fn cmp(&self, other: &Self) -> Ordering {
        other.slope.cmp(&self.slope)
    }
}

