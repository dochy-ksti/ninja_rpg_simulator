use crate::imp::structs::ai::cost_map::CostMap;
use crate::imp::structs::ai::cost_map_item::CostMapItem;
use crate::imp::structs::ai::skillmap::skill_map::SkillMap;
use crate::imp::structs::ai::skillmap::training_collection::TrainingCollection;
use crate::imp::structs::barrier::Barrier;
use crate::imp::structs::event::Event;
use crate::imp::structs::event_id::EventID;
use crate::imp::structs::events::Events;

pub(crate) fn visit(ev : &Event,
                    evs : &Events,
                    cost_map : &mut CostMap,
                    skill_map : &SkillMap,
                    iteration : u32,
                    from : Option<EventID>,
                    first : bool,
                    tcol : &mut TrainingCollection){
    //first は reached である。reachedにも一回だけvisitする必要があるので、firstではtestしない
    if first == false {
        //reached は should_visitで弾かれる。
        if should_visit(iteration, cost_map.get(ev.id())) == false { return; }
        //無限ループを避けるため、とりあえず"使用中"みたいなフラグをセット。
        // 無限ループしようとしてもshould_visitで弾かれる。
        cost_map.set_guard(ev.id(), iteration);
    }


    for w in ev.walls(){
        let mut from = from;
        for wall in w.walls(){
            if let Some(cond) = wall.cond(){
                let next = evs.get(cond.id());
                visit(next, evs, cost_map, skill_map, iteration, from);
                if cost_map.reachable(cond.id()){
                    if let Some(b) = wall.barrier(){
                        if update_cost(b, skill_map, cost_map, ev.id(), from){
                            tcol.push();
                        }
                    }
                    from = Some(cond.id());
                } else{
                    break;
                }
            } else{
                break;
            }
        }
    }
}

fn update_cost(b: &Barrier, skill_map: &SkillMap, cost_map: &mut CostMap,
               current_id: EventID, from: Option<EventID>) -> bool {
    if let Some(from_id) = from{
        cost_map.update_cost(b.id(), b.val(), current_id, Some(from_id), skill_map)
    } else{
        unimplemented!()
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