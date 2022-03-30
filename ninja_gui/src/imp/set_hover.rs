use crate::imp::control::Control;

pub(crate) fn set_hover(c : &mut dyn Control, abs_x : usize, abs_y : usize){
    let loc = c.location();
    set_rel_hover(c, abs_x as isize - loc.x() as isize,  abs_y as isize - loc.y() as isize);
}

/// returns if the coordinate is in this control,
fn set_rel_hover(c : &mut dyn Control, rel_x : isize, rel_y : isize) -> bool{
    let mut b = false;
    if let Some(children) = &mut c.children_mut(){
        for child in children.as_mut(){
            let loc = child.location();
            if set_rel_hover(child,rel_x - loc.x(), rel_y - loc.y()){
                b = true;
            }
        }
    }
    if b{
        c.set_hover(false);
        return true;
    }
    if rel_x < 0 || rel_y < 0{
        c.set_hover(false);
        return false;
    }
    if rel_x < c.size().w() as isize && rel_y < c.size().h() as isize{
        c.set_hover(true);
        return true;
    } else{
        c.set_hover(false);
        return false;
    }
}