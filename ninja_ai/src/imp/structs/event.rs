use crate::imp::structs::event_id::EventID;
use crate::imp::structs::own::Own;
use crate::imp::structs::run::Run;
use crate::imp::structs::wall::Walls;

pub struct Event {
    id : EventID,
    name : String,
    walls : Vec<Walls>,
    own : Own,
    success_common : Run,
    //fail_common : Run, あってもいいが、最小構成ではない。とりあえず最小構成で動かす

}

impl Event {
    pub fn id(&self) -> EventID{ self.id }
    pub fn name(&self) -> &str{ &self.name }
    pub fn walls(&self) -> &[Walls]{ &self.walls }
    pub fn own(&self) -> &Own{ &self.own }

}