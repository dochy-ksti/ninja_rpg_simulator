use crate::imp::structs::dstring::DString;
use crate::imp::structs::event_id::EventID;
use crate::imp::structs::nj_barrier::NjBarrier;
use crate::imp::structs::nj_cond::NjCond;

pub struct NjWalls{
    vec : Vec<NjWall>,
}

pub struct NjWall{
    desc : Option<String>,
    wall : Option<WallVal>,
    cond : Option<NjCond>,
    reserve : Vec<EventID>,
    reserve_fail : EventID>,
    run : Vec<EventID>,
    is_any : bool,
}

pub enum WallVal{
    Barrier(NjBarrier),
    Camouflage(DString),
}