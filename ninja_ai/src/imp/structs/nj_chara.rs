use crate::imp::structs::chara_id::CharaID;
use crate::imp::structs::nj_goal::NjGoals;

pub struct NjChara{
    id : CharaID,
    name : String,
    goals : Vec<NjGoals>,
}