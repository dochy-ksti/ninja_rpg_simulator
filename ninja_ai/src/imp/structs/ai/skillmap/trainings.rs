use std::cmp::Ordering;
use std::collections::BinaryHeap;
use crate::imp::structs::ai::skillmap::training::Training;
use crate::imp::structs::ai::skillmap::training_drainer::TrainingDrainer;
use crate::imp::structs::event_id::EventID;


pub(crate) struct Trainings{
    bh : BinaryHeap<Training>,
}

impl Trainings {
    pub(crate) fn peek(&self) -> Option<&Training> {
        self.bh.peek()
    }

    pub(crate) fn peek_req(&self) -> Option<u32> {
        if let Some(t) = self.peek(){
            Some(t.required())
        } else{
            None
        }
    }

    pub(crate) fn pop(&mut self) -> Option<Training> {
        self.bh.pop()
    }

    pub(crate) fn drain(&mut self) -> Option<TrainingDrainer>{
        if let Some(req) = self.peek_req(){
            Some(TrainingDrainer::new(self, req))
        } else{
            None
        }
    }
}

