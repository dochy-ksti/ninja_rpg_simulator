use crate::imp::structs::ai::skillmap::skill_map::SkillMap;
use crate::imp::structs::chara::Chara;
use crate::imp::structs::skill_id::SkillID;
use crate::imp::structs::skill_points::SkillPoints;

pub(crate) struct SkillTable{
    vec : Vec<SkillMap>
}

impl SkillTable{
    pub(crate) fn new(sps : &SkillPoints) -> SkillTable{
        let mut vec = Vec::with_capacity(SkillID::len());

        for id in 0..SkillID::len(){
            vec.push(SkillMap::new(sps.get(SkillID::new(id as u32))))
        }

        SkillTable{
            vec
        }
    }
}