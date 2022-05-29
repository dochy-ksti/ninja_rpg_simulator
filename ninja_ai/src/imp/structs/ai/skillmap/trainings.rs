use std::cmp::Ordering;
use std::collections::BinaryHeap;
use crate::imp::structs::ai::skillmap::training::Training;
use crate::imp::structs::event_id::EventID;


pub(crate) struct Trainings{
    bh : BinaryHeap<Training>,
}

impl Trainings {
    pub(crate) fn peek(&self) -> Option<&Training> {
        self.bh.peek()
    }

    pub(crate) fn pop(&mut self) -> Option<Training> {
        self.bh.pop()
    }
}

