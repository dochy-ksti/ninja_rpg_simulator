use std::num::NonZeroU32;
use crate::imp::structs::event_id::EventID;

pub(crate) struct SkillMapItem {
    stacked_skill_point : u32,
    stacked_distance : u32,
    repeatable_increase : Option<NonZeroU32>,
    event_id : EventID,
}

impl SkillMapItem {
    pub(crate) fn new(
        stacked_skill_point: u32,
        stacked_distance: u32,
        repeatable_increase: Option<NonZeroU32>,
        event_id: EventID) -> SkillMapItem {
        SkillMapItem {
            stacked_skill_point,
            stacked_distance,
            repeatable_increase,
            event_id,
        }
    }

    pub(crate) fn stacked_skill_point(&self) -> u32 { self.stacked_skill_point }
    pub(crate) fn stacked_distance(&self) -> u32 { self.stacked_distance }
    pub(crate) fn repeatable_increase(&self) -> Option<NonZeroU32> { self.repeatable_increase }
    pub(crate) fn event_id(&self) -> EventID { self.event_id }
}

