use crate::imp::structs::ai::skillmap::available_trainings::AvailableTraining;
use crate::imp::structs::ai::skillmap::skill_map_item::SkillMapItem;

pub(crate) struct SkillMap{
    map : Vec<SkillMapItem>,
    current_skill_point : u32,
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
    pub(crate) fn set_repeatable_training(&mut self, at : AvailableTraining) -> bool{
        if let Some(last) = self.last(){
            if last.event_id() == at.training_ref().event_id(){
                return false;
            }
        }
        let item = SkillMapItem::new(
            self.stacked_skill_point(),
            self.stacked_distance(),
            Some(at.training_ref().increase()),
            at.training_ref().event_id(),
        );
        self.map.push(item);
    }

    pub(crate) fn set_training(&mut self, at : AvailableTraining) -> u32{
        let item = SkillMapItem::new(
            self.stacked_skill_point() + at.training_ref().increase(),
            self.stacked_distance() + at.training_ref().distance(),
            None,
            at.training_ref().event_id(),
        );
        self.map.push(item);
        self.stacked_skill_point()
    }
}





