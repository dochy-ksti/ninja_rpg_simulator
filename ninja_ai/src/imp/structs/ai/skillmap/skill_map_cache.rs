use std::collections::BTreeMap;
use ordered_float::NotNan;
use crate::imp::structs::ai::skillmap::skill_map::SkillMap;
use crate::imp::structs::ai::skillmap::skill_map_item::SkillMapItem;
use crate::imp::structs::ai::skillmap::slope::Slope;

pub(crate) struct SkillMapCache{
    vec : BTreeMap<u32, SkillMapItem>
}

pub(crate) struct AvailableTrainings<'a >{
    vec : BTreeMap<Slope, &'a SkillMapItem>
}

impl SkillMapCache{
    pub(crate) fn into_map(self) -> SkillMap{
        unimplemented!()
    }
}