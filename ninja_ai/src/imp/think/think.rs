use crate::imp::structs::ai::skillmap::skill_table::SkillTable;
use crate::imp::structs::chara::Chara;
use crate::imp::structs::chara_id::CharaID;
use crate::imp::structs::charas::Charas;
use crate::imp::structs::event::Event;
use crate::imp::structs::event_id::EventID;
use crate::imp::structs::events::Events;

pub fn think(charas : &Charas, _evs : &Events, pc_id : CharaID) -> EventID{
    let pc = charas.get(pc_id);
    let mut st = SkillTable::new(pc.skill_points());
    unimplemented!()

}