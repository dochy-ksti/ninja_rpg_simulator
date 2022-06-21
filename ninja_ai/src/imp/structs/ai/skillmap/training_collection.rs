use std::iter;
use crate::imp::structs::ai::skillmap::training::Training;
use crate::imp::structs::event::Event;
use crate::imp::structs::event_id::EventID;
use crate::imp::structs::own::Own;
use crate::imp::structs::skill_id::SkillID;
use crate::imp::structs::train_skill::AddSkillPoint;

pub(crate) struct TrainingCollection{
    vec : Vec<Vec<Training>>
}

impl TrainingCollection{
    pub(crate) fn new() -> TrainingCollection{
        TrainingCollection{
            vec : iter::repeat_with(|| Vec::new()).take(SkillID::len()).collect()
        }
    }

    pub(crate) fn push_reached_event(&mut self, ev : &Event){
        if ev.own().repeatable(){
            if let Some(asp) = ev.add_skill_point(){
                self.push(asp, true, 0, 0, ev.id())
            }
        }
    }

    pub(crate) fn push(&mut self, asp : AddSkillPoint, repeatable : bool, required : u32, distance : u32, id : EventID) {
        self.vec.get_mut(asp.id().num() as usize).unwrap()
            .push(Training::new(repeatable, asp.point(), required, distance, id))
    }
}