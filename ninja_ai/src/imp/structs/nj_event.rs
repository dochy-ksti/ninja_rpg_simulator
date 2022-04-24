use crate::imp::structs::ev_id::EvID;
use crate::imp::structs::nj_chain::NjChain;
use crate::imp::structs::nj_own::NjOwn;
use crate::imp::structs::nj_wall::NjWalls;

pub struct NjEvent{
    id : u32,
    name : String,
    et : EventType,
}

pub enum EventType{
    Wall(WallEvent),
    Flag(FlagEvent),
}

///WallEventは可視性も実行可能性もWallで定義できる
pub struct WallEvent{
    walls : Vec<NjWalls>,
    own : NjOwn,
}

/// FlagEventは条件を満たした場合にsetされる。
/// 可視性は、デフォルトでは、setされるまで見えないが、setされると見えるようになる
pub struct FlagEvent{
    description : Option<String>,
    visible : Option<NjChain>,
    set : NjChain,
}

impl NjEvent{
    pub fn id(&self) -> u32{ self.id }
}