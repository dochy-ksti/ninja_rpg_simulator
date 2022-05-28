use crate::imp::structs::skill_id::SkillID;

pub(crate) struct RequiredSkills{
    map : Vec<u32>
}

impl RequiredSkills{
    pub(crate) fn new() -> RequiredSkills{ RequiredSkills{ map : Vec::with_capacity(SkillID::len()) } }
    pub(crate) fn overwrite(&mut self, r : &RequiredSkills){
        self.map.copy_from_slice(&r.map);
    }

    pub(crate) fn set(&mut self, id : SkillID, val : u32){
        unsafe {
            *self.map.get_mut_unchecked(id.num()) = val;
        }
    }
}

