use crate::imp::structs::chara_id::CharaID;
use crate::imp::structs::goal::Goals;

pub struct Chara {
    id : CharaID,
    name : String,
    goals : Vec<Goals>,
}