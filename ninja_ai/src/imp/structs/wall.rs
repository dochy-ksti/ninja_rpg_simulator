use crate::imp::structs::dstring::DString;
use crate::imp::structs::barrier::Barrier;
use crate::imp::structs::cond::Cond;

pub struct Walls {
    vec : Vec<Wall>,
}

pub struct Wall {
    desc : Option<String>,
    wall : Option<WallVal>,
    cond : Option<Cond>,
}

pub enum WallVal{
    Barrier(Barrier),
    Camouflage(DString),
}