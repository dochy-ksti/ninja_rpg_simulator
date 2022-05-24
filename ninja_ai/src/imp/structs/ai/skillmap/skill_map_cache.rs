use crate::imp::structs::ai::skillmap::skill_map::SkillMap;
use crate::imp::structs::ai::skillmap::skill_map_item::SkillMapItem;

pub(crate) struct SkillMapCache{
    vec : Vec<SkillMapItem>
}

impl SkillMapCache{
    pub(crate) fn into_map(self) -> SkillMap{
        unimplemented!()
    }
}