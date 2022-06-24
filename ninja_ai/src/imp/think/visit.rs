use crate::imp::structs::ai::cost_map::CostMap;
use crate::imp::structs::ai::cost_map_item::CostMapItem;
use crate::imp::structs::ai::skillmap::skill_map::SkillMap;
use crate::imp::structs::barrier::Barrier;
use crate::imp::structs::event::Event;
use crate::imp::structs::event_id::EventID;
use crate::imp::structs::events::Events;

pub(crate) fn visit(ev : &Event, evs : &Events, cost_map : &mut CostMap, skill_map : &SkillMap, iteration : u32, from : Option<EventID>){
    if should_visit(iteration, cost_map.get(ev.id())) == false{ return; }
    //無限ループを避けるため、とりあえずunreachableをセット
    cost_map.set_unreachable(ev.id(), iteration);

    for w in ev.walls(){
        let mut from = from;
        for wall in w.walls(){
            if let Some(b) = wall.barrier(){
                update_cost(b, skill_map, cost_map, ev.id(), from);
            }

            if let Some(cond) = wall.cond(){
                cond.id()
            }
        }
    }
}

fn update_cost(b: &Barrier, skill_map: &SkillMap, cost_map: &mut CostMap, current_id: EventID, from: Option<EventID>) {
    todo!() aaa
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