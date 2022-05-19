use crate::imp::structs::event_id::EventID;
use crate::imp::structs::barrier::Barrier;

pub(crate) enum LineItem{
    ID(EventID),
    InvisibleWall(EventID),
    WithBarrier(WithBarrier)
}

pub(crate) struct WithBarrier{
    barrier : Barrier,
    id : EventID,
}