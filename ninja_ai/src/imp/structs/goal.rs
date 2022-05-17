use crate::imp::structs::dstring::DString;
use crate::imp::structs::event_id::EventID;
use crate::imp::structs::cond::Cond;

pub struct Goals {
    vec : Vec<Goal>
}

pub struct Goal {
    desc : Option<String>,
    goal : Option<GoalVal>,
    cond : Cond,
    is_any : bool,
}

pub enum GoalVal{
    Camouflage(DString),
    Event(EventID),
}