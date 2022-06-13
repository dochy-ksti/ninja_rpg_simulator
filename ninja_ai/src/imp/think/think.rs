use crate::imp::structs::ai::skillmap::skill_table::SkillTable;
use crate::imp::structs::chara::Chara;
use crate::imp::structs::event::Event;

pub fn think(charas : &[Chara], _evs : &[Event], pc_id : u32) -> u32{
    let pc = &charas[pc_id as usize];
    let st = SkillTable::new(pc.skill_points());
    unimplemented!()

}