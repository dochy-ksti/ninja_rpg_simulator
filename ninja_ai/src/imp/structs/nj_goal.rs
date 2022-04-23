use crate::imp::structs::dstring::DString;
use crate::imp::structs::nj_cond::NjCond;

pub struct NjGoals{
    vec : Vec<NjGoal>
}

pub struct NjGoal{
    desc : Option<String>,
    goal : Option<GoalVal>,
    cond : NjCond,
    is_any : bool,
}

pub enum GoalVal{
    Camouflage(DString),
    Event(String),
}