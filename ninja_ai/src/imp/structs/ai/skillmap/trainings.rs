use std::cmp::Ordering;
use std::collections::binary_heap::BinaryHeap;
use crate::imp::structs::ai::skillmap::available_trainings::{AvailableTraining, AvailableTrainings};
use crate::imp::structs::ai::skillmap::slope::Slope;
use crate::imp::structs::ai::skillmap::training::Training;
use crate::imp::structs::ai::skillmap::training_drainer::TrainingDrainer;
use crate::imp::structs::event_id::EventID;


pub(crate) struct Trainings{
    bh : BinaryHeap<Training>,
    average_increase : u32,
}

impl Trainings {
    pub(crate) fn new(trainings : Vec<Training>) -> Option<Trainings>{
        let mut bh : BinaryHeap<Training> = BinaryHeap::with_capacity(trainings.len());
        let mut sum_inc : u64 = 0;
        let mut count : usize = 0;
        for t in trainings{
            count += 1;
            if t.repeatable() == false {
                sum_inc += t.increase() as u64;
            } else{
                //repeatableは5回ぐらい平均的にやるものと考える
                //あまりにも適当だが、いいアイデアがない
                sum_inc += t.increase() as u64 * 5;
            }

            bh.push(t);
        }
        if count != 0 {
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

    /// achieved 以下のアイテムを availableに全部移す
    /// achievedがない場合、req値最小のアイテムをうつす。複数あれば複数。 achievedがない場合というのはつまりrepeatableで限度がない状態
    pub(crate) fn move_items(&mut self, availables : &mut AvailableTrainings, req : u32) -> bool {
        let mut modified = false;
        while let Some(r) = self.peek_req() {
            if r <= req {
                let tr = self.pop().unwrap();
                let slope = tr.calc_slope(self.average_increase);
                let av = AvailableTraining::new(tr, slope);
                availables.push(av);
                modified = true;
            } else{
                break;
            }
        }
        modified
    }

    pub(crate) fn move_sloped(&mut self, availables : &mut AvailableTrainings, needed : Slope) -> bool{
        while let Some(t) = self.peek() {
            let slope = t.calc_slope(self.average_increase);
            if needed < slope{
                let required = t.required();
                return move_items(self, availables, required);
            } else{
                self.pop();
            }
        }
        return false;
    }
}

