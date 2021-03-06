use crate::imp::structs::chara_id::CharaID;
use crate::imp::structs::event_id::EventID;
use crate::imp::structs::goal::Goals;
use crate::imp::structs::owned_events::OwnedEvents;
use crate::imp::structs::skill_points::SkillPoints;

pub struct Chara {
    id : CharaID,
    name : String,
    goals : Vec<Goals>,
    skill_points : SkillPoints,
    own : OwnedEvents,
}

impl Chara{
    pub fn new(
        id : CharaID,
        name : String,
        goals : Vec<Goals>,
        skill_points : SkillPoints,
        own : OwnedEvents) -> Chara{
        Chara{ id, name, goals, skill_points, own }
    }

    pub fn id(&self) -> CharaID{ self.id }
    pub fn name(&self) -> &str{ &self.name }
    pub fn goals(&self) -> &[Goals]{ &self.goals }
    pub fn skill_points(&self) -> &SkillPoints{ &self.skill_points }
    pub fn own(&self) -> &OwnedEvents{ &self.own }

}
