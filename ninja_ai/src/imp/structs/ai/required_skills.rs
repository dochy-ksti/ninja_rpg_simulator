pub(crate) struct RequiredSkills{
    map : Vec<u32>
}

impl RequiredSkills{
    pub(crate) fn new(skill_len : usize) -> RequiredSkills{ RequiredSkills{ map : Vec::with_capacity(skill_len) } }
}

