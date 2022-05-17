use crate::imp::structs::event_id::EventID;
use crate::imp::structs::barrier::Barrier;

pub(crate) struct LineItem{
    id : EventID,
    barrier : Barrier
}