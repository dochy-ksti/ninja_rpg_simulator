use crate::imp::structs::event_id::EventID;
use crate::imp::structs::nj_chain::NjChain;
use crate::imp::structs::own::Own;
use crate::imp::structs::run::Run;
use crate::imp::structs::wall::Walls;
use crate::imp::structs::train_skill::TrainSkill;

pub struct Event {
    id : EventID,
    name : String,
    walls : Vec<Walls>,
    own : Own,
    success_common : Run,
    fail_common : Run,

}

impl Event {
    pub fn id(&self) -> EventID{ self.id }
}