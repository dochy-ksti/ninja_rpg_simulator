use crate::imp::structs::ai::cost_map::CostMap;
use crate::imp::structs::ai::cost_map_item::CostMapItem;
use crate::imp::structs::ai::skillmap::skill_map::SkillMap;
use crate::imp::structs::event::Event;
use crate::imp::structs::event_id::EventID;
use crate::imp::structs::events::Events;

pub(crate) fn visit(ev : &Event, evs : &Events, cost_map : &mut CostMap, skill_map : &SkillMap, iteration : u32){
    if
    for w in ev.walls(){
        for wall in w.walls(){
            if let Some(cond) = wall.cond(){
                cond.id()
            }
        }
    }
}

fn should_visit(iteration : u32, item : &CostMapItem) -> bool{
    if let Some(item) = item.item(){
        if item.is_reached(){
            //到達済み
            false
        } else if item.iteration() < iteration{
            //過去のiterationで計算したが今回はまだ計算していない
            true
        } else{
            false
        }
    } else{
        //まだ到達していない
        true
    }
}