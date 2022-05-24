use std::collections::BTreeMap;
use crate::imp::structs::ai::skillmap::skill_map_item::SkillMapItem;

pub(crate) struct SkillMap{
    map : BTreeMap<u32, SkillMapItem>
}

impl SkillMap{
    pub(crate) fn cost(&self, sp : u32) -> u32{

    }
}



