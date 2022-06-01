use std::cmp::Ordering;
use std::collections::BinaryHeap;
use crate::imp::structs::ai::skillmap::slope::Slope;
use crate::imp::structs::ai::skillmap::training::Training;

pub(crate) struct AvailableTrainings{
    bh : BinaryHeap<AvailableTraining>,
    current_repeatable : Option<AvailableTraining>,
}

impl AvailableTrainings{
    pub(crate) fn new() -> AvailableTrainings{
        AvailableTrainings{ bh : BinaryHeap::new(), current_repeatable : None }
    }
    pub(crate) fn push(&mut self, at : AvailableTraining){
        self.bh.push(at)
    }
    pub(crate) fn current_repeatable(&mut self) -> Option<&AvailableTraining>{
        self.current_repeatable.as_ref()
    }
    pub(crate) fn set_current_repeatable(&mut self, at : AvailableTraining){
        self.current_repeatable = Some(at);
    }
    pub(crate) fn peek(&self) -> Option<&AvailableTraining>{
        self.bh.peek()
    }
    pub(crate) fn pop(&mut self) -> Option<AvailableTraining>{
        self.bh.pop()
    }
    pub(crate) fn clear_heap(&mut self){
        self.bh.clear()
    }
}

#[derive(Debug)]
pub(crate) struct AvailableTraining{
    slope : Slope,
    training : Training,
}

impl AvailableTraining{
    pub(crate) fn from_training(training : Training) -> AvailableTraining{
        if training.repeatable(){

        }
    }
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
        self.slope.cmp(&other.slope)
    }
}

