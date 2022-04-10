use std::sync::Arc;
use crate::imp::control::Control;
use crate::imp::structs::gui_id::GuiID;

pub(crate) fn find_ctl_mut<'a, 'b>(c : &'a mut (dyn Control + 'static), id : &'b GuiID) -> Option<&'a mut (dyn Control + 'static)>{
    if c.id() == id {
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