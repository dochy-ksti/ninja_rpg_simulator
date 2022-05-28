use std::collections::BTreeMap;
use ordered_float::NotNan;
use crate::imp::structs::ai::required_skills::RequiredSkills;

pub(crate) struct Training{
    repeatable : bool,
    increase : u32,
    required : u32,
    distance : usize,
}

pub(crate) struct Trainings{
    btree : BTreeMap<u32, Training>,
}

impl Trainings {
    pub(crate) fn peek(&self) -> Option<(&u32, &Training)> {
        self.btree.iter().next()
    }

    pub(crate) fn take(&mut self) -> Option<(u32, Training)> {
        if let Some((&require,_)) = self.peek(){
            self.btree.remove(&require).map(|t| (require, t))
        } else{
            None
        }
    }
}