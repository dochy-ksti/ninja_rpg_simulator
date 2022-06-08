
use crate::imp::structs::ai::skillmap::skill_map_item::SkillMapItem;

pub(crate) struct SkillMap{
    map : Vec<SkillMapItem>
}

impl SkillMap{
    pub(crate) fn new() -> SkillMap{
        SkillMap{ map : Vec::new() }
    }

    pub(crate) fn push(&mut self, item : SkillMapItem){
        self.map.push(item);
    }
}





