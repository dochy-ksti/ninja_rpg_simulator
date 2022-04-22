use crate::imp::structs::dstring::DString;
use crate::imp::structs::nj_barrier::NjBarrier;
use crate::imp::structs::nj_cond::NjCond;

pub struct NjWalls{
    vec : Vec<NjWall>,
}

pub struct NjWall{
    name : Option<DString>,
    desc : Option<String>,
    wall : Option<NjBarrier>,
    cond : Option<NjCond>,
    reserve : Option<String>,
    run : Option<String>,
}