use std::collections::{BTreeSet};
use crate::imp::structs::ai::skillmap::skill_map_item::SkillMapItem;

pub(crate) struct SkillMap{
    map : BTreeSet<SkillMapItem>
}

impl SkillMap{
    pub(crate) fn new() -> SkillMap{
        SkillMap{ map : BTreeSet::new() }
    }

    pub(crate) fn set(&mut self, item : SkillMapItem){
        self.map.insert(item);
    }
}





