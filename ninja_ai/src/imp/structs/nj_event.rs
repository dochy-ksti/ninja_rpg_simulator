use crate::imp::structs::event_id::EventID;
use crate::imp::structs::nj_chain::NjChain;
use crate::imp::structs::nj_own::NjOwn;
use crate::imp::structs::nj_run::NjRun;
use crate::imp::structs::nj_wall::NjWalls;
use crate::imp::structs::train_skill::TrainSkill;

pub struct NjEvent{
    id : EventID,
    name : String,
    walls : Vec<NjWalls>,
    own : NjOwn,
    success_common : NjRun,
    fail_common : NjRun,

}

impl NjEvent{
    pub fn id(&self) -> EventID{ self.id }
}