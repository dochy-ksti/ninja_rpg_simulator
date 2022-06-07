use std::cmp::Ordering;
use std::collections::BinaryHeap;
use crate::imp::structs::ai::skillmap::training::Training;
use crate::imp::structs::ai::skillmap::training_drainer::TrainingDrainer;
use crate::imp::structs::event_id::EventID;


pub(crate) struct Trainings{
    bh : BinaryHeap<Training>,
    average_increase : u32,
}

impl Trainings {
    pub(crate) fn new(trainings : impl Iterator<Item=Training>) -> Option<Trainings>{
        let mut bh : BinaryHeap<Training> = BinaryHeap::with_capacity(trainings.size_hint().0);
        let mut sum_inc : u64 = 0;
        let mut sum_dis : u64 = 0;
        let mut count : usize = 0;
        for t in trainings{
            count += 1;
            if t.repeatable() == false {
                sum_inc += t.increase() as u64;
            }
            sum_dis += t.increase() as u64;
            bh.push(t);
        }
        if count != 0 {
            let avg_dis = u64::max(sum_dis / count as u64, 1);
            for t in bh.iter() {
                if t.repeatable() {
                    //avg_dis と increase を掛ける意味はよくわからないが、とにかくなんらかの代表値を用意しなければならないため
                    //苦肉の策で2つをかけ合わせている
                    sum_inc += avg_dis * t.increase() as u64;
                }
            }
            let avg_inc = u64::max(sum_inc / count as u64, 1);
            Some(Trainings{ bh, average_increase : avg_inc as u32 })
        } else{
            None
        }
    }

    /// トレーニングをやったときにどのくらい伸びるかの平均値のようなもの。1以上
    pub(crate) fn average_increase(&self) -> u32{ self.average_increase }

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

