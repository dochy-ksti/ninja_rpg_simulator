use crate::imp::structs::event_id::EventID;
use crate::imp::structs::nj_chain::NjChain;
use crate::imp::structs::nj_own::NjOwn;
use crate::imp::structs::nj_wall::NjWalls;
use crate::imp::structs::train_skill::TrainSkill;

pub struct NjEvent{
    id : EventID,
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
/// 可視性は、見える形でreserveされたり、見えるイベントからsetやcondで見える形で参照されれば、見えるようになる
pub struct FlagEvent{
    description : Option<String>,
    set : NjChain,
    run : Vec<EventID>,
    train_skill : Option<TrainSkill>,
    own : NjOwn,
}

impl NjEvent{
    pub fn id(&self) -> EventID{ self.id }
}