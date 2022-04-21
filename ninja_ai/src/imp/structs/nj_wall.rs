use crate::imp::structs::dstring::DString;
use crate::imp::structs::nj_barrier::NjBarrier;

pub struct NjWalls{

}

pub struct NjWall{
    name : DString,
    desc : Option<String>,
    wall : Option<NjBarrier>
}