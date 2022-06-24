use crate::imp::structs::dstring::DString;
use crate::imp::structs::barrier::Barrier;
use crate::imp::structs::cond::Cond;

pub struct Walls {
    vec : Vec<Wall>,
}

pub struct Wall {
    desc : Option<String>,
    wall : WallVal,
    cond : Option<Cond>,
}

pub enum WallVal{
    Barrier(Barrier),
    Camouflage(DString),
    None
}

impl Walls{
    pub fn walls(&self) -> &[Wall]{ &self.vec }
}

impl Wall{
    pub fn desc(&self) -> Option<&str>{ self.desc.as_deref() }
    pub fn wall(&self) -> &WallVal{ &self.wall }
    pub fn barrier(&self) -> Option<&Barrier>{
        match &self.wall{
            WallVal::Barrier(b) => Some(b),
            _ => None,
        }
    }
    pub fn cond(&self) -> Option<&Cond>{ self.cond.as_ref() }
}