use std::sync::Arc;
use crate::imp::control::Control;
use crate::imp::structs::gui_id::GuiID;

pub(crate) fn find_hoverd_ctl<'a>(c : &'a (dyn Control + 'static), abs_x : usize, abs_y : usize) -> Option<&'a GuiID>{
    let loc = c.location();
    rel_hover(c, abs_x as isize - loc.x() as isize,  abs_y as isize - loc.y() as isize)
}

// returns if the coordinate is in this control,
fn rel_hover<'a>(c : &'a (dyn Control + 'static), rel_x : isize, rel_y : isize) -> Option<&'a GuiID>{
    if let Some(mut children) = c.children(){
        for child in children.as_mut(){
            let loc = child.location();
            if let Some(r) = rel_hover(child,rel_x - loc.x(), rel_y - loc.y()){
                return Some(r);
            }
        }
    }

    if rel_x < 0 || rel_y < 0{
        return None;
    }
    if rel_x < c.size().w() as isize && rel_y < c.size().h() as isize{
        return Some(c.id());
    } else{
        return None;
    }
}