use std::cmp::Ordering;
use std::num::NonZeroU32;
use crate::imp::structs::ai::skillmap::available_trainings::AvailableTraining;
use crate::imp::structs::ai::skillmap::skill_map_item::SkillMapItem;
use crate::imp::structs::ai::skillmap::training::Training;
use crate::imp::structs::event_id::EventID;
use crate::imp::structs::skill_id::SkillID;

pub(crate) struct SkillMap{
    map : Vec<SkillMapItem>,
    current_skill_point : u32,
}

impl SkillMap {

}

impl SkillMap{
    pub(crate) fn new(current_skill_point : u32) -> SkillMap{
        SkillMap{ map : Vec::new(), current_skill_point }
    }

    fn last(&self) -> Option<&SkillMapItem>{
        self.map.last()
    }

    fn stacked_skill_point(&self) -> u32{
        if let Some(item) = self.last(){
            item.stacked_skill_point()
        } else{
            self.current_skill_point
        }
    }

    fn stacked_distance(&self) -> u32{
        if let Some(item) = self.last(){
            item.stacked_distance()
        } else{
            0
        }
    }

    ///同じrepeatable trainingを二度連続で突っ込もうとすると失敗する
    pub(crate) fn set_repeatable_training(&mut self, t : Training) -> bool{
        let (stacked_skill_point, count) = if let Some(last) = self.last(){
            if last.event_id() == t.event_id(){
                return false;
            }
            if let Some(inc) = last.repeatable_increase(){
                //repeatableの連続。ここには調節が必要になる
                debug_assert!(last.stacked_skill_point() < t.required());
                let inc = inc.get();
                let diff = t.required() - last.stacked_skill_point();
                let count = diff / inc + if diff % inc == 0{ 0 } else { 1 };
                (last.stacked_skill_point() + inc * count, count)
            } else{
                (last.stacked_skill_point(), 0)
            }
        } else{ (0,0) };

        let item = SkillMapItem::new(
            stacked_skill_point,
            self.stacked_distance() + t.distance() + count,
            Some(NonZeroU32::new(t.increase()).unwrap()),
            t.event_id(),
        );
        self.map.push(item);
        return true;
    }

    pub(crate) fn set_training(&mut self, t : Training) -> u32{
        let (stacked_skill_point, count) = if let Some(last) = self.last(){
            if let Some(inc) = last.repeatable_increase(){
                let inc = inc.get();
                debug_assert!(last.stacked_skill_point() < t.required());
                let diff = t.required() - last.stacked_skill_point();
                let count = diff / inc + if diff % inc == 0{ 0 } else { 1 };
                (last.stacked_skill_point() + inc * count, count)
            } else{
                (last.stacked_skill_point(), 0)
            }
        } else{ (0,0) };

        //repeatableの場合、requireまでやって、そこからonceのtrainingをやりました、というルートと、
        //require以降もrepeatableをやりましたというルートがあって、どっちがdistanceが小さいか比較して選ぶ必要があると思う
        //そのインチキがコスト計算をおかしくしてしまう可能性もあると思うがどうなんだろうな
        let item = SkillMapItem::new(
            stacked_skill_point + t.increase(),
            self.stacked_distance() + count + t.distance(),
            None,
            t.event_id(),
        );
        self.map.push(item);
        self.stacked_skill_point()
    }



    pub(crate) fn get_cost(&self, required_skill_point : u32) -> SkillMapCost{
        if required_skill_point <= self.current_skill_point{ SkillMapCost::Reached }
        else {
            match self.get_cost_inner(required_skill_point) {
                Some((dis, id)) => SkillMapCost::Reachable(dis, id),
                None => SkillMapCost::Unreachable,
            }
        }
    }

    fn get_cost_inner(&self, required_skill_point : u32) -> Option<(u32, EventID)>{
        match self.map.binary_search_by(|item| {
            let ord = item.stacked_skill_point().cmp(&required_skill_point);
            if ord != Ordering::Equal{ return ord; }
            return if item.repeatable_increase().is_some() { Ordering::Less } else { Ordering::Equal }
        }){
            Ok(idx) => {
                let item = self.map.get(idx).unwrap();
                Some((item.stacked_distance(), item.event_id()))
            }
            Err(idx) =>{
                let right = if let Some(right) = self.map.get(idx){
                    if right.repeatable_increase().is_none(){
                        Some((right.stacked_distance(), right.event_id()))
                    } else{ None }
                } else { None };
                let left = if let Some(left) = self.map.get(idx - 1){
                    if let Some(inc) = left.repeatable_increase(){
                        let inc = inc.get();
                        debug_assert!(left.stacked_skill_point() < required_skill_point);
                        let diff = required_skill_point - left.stacked_skill_point();
                        let count = diff / inc + if diff % inc == 0{ 0 } else { 1 };
                        Some((left.stacked_distance() + count, left.event_id()))
                    } else{ None }
                } else { None };
                if let Some(right) = right{
                    if let Some(left) = left{
                        if left.0 < right.0{ Some(left) } else { Some(right) }
                    } else{ Some(right) }
                } else if let Some(left) = left{
                    Some(left)
                } else {
                    None
                }
            }
        }
    }
}
pub(crate) enum SkillMapCost{
    Reachable(u32, EventID),
    Reached,
    Unreachable
}




