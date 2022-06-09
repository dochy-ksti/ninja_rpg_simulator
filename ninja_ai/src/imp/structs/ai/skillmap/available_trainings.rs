use std::cmp::Ordering;
use std::collections::binary_heap::BinaryHeap;
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

    /// repeatableよりslopeが小さい場合、pushは意味ないからしない
    pub(crate) fn push(&mut self, at : AvailableTraining) -> bool{
        if let Some(repeatable) = &self.current_repeatable{
            if at.slope <= repeatable.slope{ return false; }
        }
        self.bh.push(at);
        return true;
    }
    pub(crate) fn current_repeatable(&mut self) -> Option<&AvailableTraining>{
        self.current_repeatable.as_ref()
    }
    pub(crate) fn set_current_repeatable(&mut self, t : Training){
        self.current_repeatable = Some({
            let slope = Slope::new(t.increase(), 1);
            AvailableTraining::new(t, slope)
        });
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

#[derive(Debug, Clone)]
pub(crate) struct AvailableTraining{
    training : Training,
    slope : Slope,
}

impl AvailableTraining {
    pub(crate) fn new(training: Training, slope : Slope) -> AvailableTraining {
        AvailableTraining{ training, slope }
    }
    pub(crate) fn slope(&self) -> Slope{ self.slope }
    pub(crate) fn training(self) -> Training{ self.training }
    pub(crate) fn training_ref(&self) -> &Training{ &self.training }
    pub(crate) fn repeatable(&self) -> bool{ self.training.repeatable() }
}

impl PartialEq for AvailableTraining{
    fn eq(&self, other: &Self) -> bool {
        self.training.cmp(othe) == Ordering::Equal
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

