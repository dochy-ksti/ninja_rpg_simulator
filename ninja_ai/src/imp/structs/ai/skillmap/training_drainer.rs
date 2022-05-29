use crate::imp::structs::ai::skillmap::training::{Training};
use crate::imp::structs::ai::skillmap::trainings::Trainings;

pub(crate) struct TrainingDrainer<'a>{
    trainings : &'a mut Trainings,
    required : u32,
}

impl<'a> TrainingDrainer<'a>{
    pub fn new(trainings : &'a mut Trainings, required : u32) -> TrainingDrainer{ TrainingDrainer{ trainings, required }}

    pub(crate) fn next(&mut self) -> Option<Training>{
        if let Some(t) = self.trainings.peek(){
            if t.required() <= self.required{
                self.trainings.pop()
            } else{ None }
        } else{ None }
    }
}