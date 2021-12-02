use std::collections::HashMap;
use crate::imp::topology::event::Event;

pub(crate) struct EventMap{
    map : HashMap<usize, Event>
}