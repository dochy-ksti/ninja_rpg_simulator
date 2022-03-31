use std::sync::Arc;
use crate::imp::control::Control;

pub(crate) fn find_ctl_mut<'a, 'b>(c : &'a mut (dyn Control + 'static), id : &'b Arc<()>) -> Option<&'a mut (dyn Control + 'static)>{
    if c.id().eq(id) {
        return Some(c);
    }
    if let Some(mut children) = c.children_mut(){
        for child in children.as_mut() {
            if let Some(r) = find_ctl_mut(child, id) {
                return Some(r);
            }
        }
    }
    return None;
}