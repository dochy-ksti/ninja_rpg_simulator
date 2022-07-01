use crate::imp::structs::skill_id::SkillID;

pub(crate) struct RequiredSkills{
    map : Vec<u32>
}

impl RequiredSkills{
    pub(crate) fn empty() -> RequiredSkills{ RequiredSkills{ map : Vec::new() } }
    pub(crate) fn maxed() -> RequiredSkills{
        RequiredSkills{
            map : std::iter::repeat(u32::MAX).take(SkillID::len()).collect()
        }
    }
    pub(crate) fn is_empty(&self) -> bool{ self.map.is_empty() }

    pub(crate) fn overwrite(&mut self, r : &RequiredSkills){
        self.map.copy_from_slice(&r.map);
    }

    pub(crate) fn set(&mut self, id : SkillID, val : u32){
        unsafe {
            *self.map.get_unchecked_mut(id.num() as usize) = val;
        }
    }

    pub(crate) fn get(&self, id : SkillID) -> u32{
        unsafe {
            *self.map.get_unchecked(id.num() as usize)
        }
    }
}

