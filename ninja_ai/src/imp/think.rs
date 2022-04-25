use crate::imp::structs::nj_barrier::NjSkill;
use crate::imp::structs::nj_chara::NjChara;
use crate::imp::structs::nj_event::NjEvent;

pub fn think(charas : &[NjChara], evs : &[NjEvent], pc_id : u32) -> u32{
    let hoge = NjSkill::new(2);
    hoge.val()

}