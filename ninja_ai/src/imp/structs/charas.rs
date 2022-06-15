use crate::imp::structs::chara::Chara;
use crate::imp::structs::chara_id::CharaID;

pub struct Charas{
    vec : Vec<Chara>
}

impl Charas{
    pub fn new(vec : Vec<Chara>) -> Charas{ Charas{ vec } }
    pub fn get(&self, id : CharaID) -> &Chara{ self.vec.get(id.num() as usize).unwrap() }
}