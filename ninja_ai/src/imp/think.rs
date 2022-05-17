use crate::imp::structs::barrier::NjSkill;
use crate::imp::structs::chara::Chara;
use crate::imp::structs::event::Event;

pub fn think(charas : &[Chara], evs : &[Event], pc_id : u32) -> u32{
    let hoge = NjSkill::new(2);
    hoge.val()

}