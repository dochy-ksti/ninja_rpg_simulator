use crate::imp::structs::event_id::EventID;
use crate::imp::structs::own::Own;
use crate::imp::structs::run::Run;
use crate::imp::structs::train_skill::AddSkillPoint;
use crate::imp::structs::wall::Walls;

pub struct Event {
    id : EventID,
    name : String,
    walls : Vec<Walls>,
    own : Own,
    add_skill_point : Option<AddSkillPoint>,
}

impl Event {
    pub fn id(&self) -> EventID{ self.id }
    pub fn name(&self) -> &str{ &self.name }
    pub fn walls(&self) -> &[Walls]{ &self.walls }
    pub fn own(&self) -> Own{ self.own }
    pub fn add_skill_point(&self) -> Option<AddSkillPoint>{ self.add_skill_point }

}