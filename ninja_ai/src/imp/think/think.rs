use crate::imp::structs::ai::cost_map::CostMap;
use crate::imp::structs::ai::skillmap::skill_table::SkillTable;
use crate::imp::structs::ai::skillmap::training_collection::TrainingCollection;
use crate::imp::structs::chara::Chara;
use crate::imp::structs::chara_id::CharaID;
use crate::imp::structs::charas::Charas;
use crate::imp::structs::event::Event;
use crate::imp::structs::event_id::EventID;
use crate::imp::structs::events::Events;

pub fn think(charas : &Charas, evs : &Events, pc_id : CharaID) -> EventID{
    let pc = charas.get(pc_id);
    let mut st = SkillTable::new(pc.skill_points());
    let mut tcol = TrainingCollection::new();
    let mut costs = CostMap::new(evs.len());
    let own = pc.own();
    for (id, own) in own.iter(){
        if own{
            costs.reached(id);
            tcol.push_reached_event(evs.get(id))
        }
    }
    for i in 0..evs.len(){
        let id = EventID::new(i as u32);
        if costs.reached(id){
            //visit()
        }
    }

    unimplemented!()

}