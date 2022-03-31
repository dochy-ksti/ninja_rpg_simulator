use std::sync::Arc;
use crate::imp::control::Control;
use crate::imp::find_ctl::find_ctl_mut;
use crate::imp::find_hovered_ctl::find_hoverd_ctl;
use crate::imp::structs::vert_panel::VertPanel;

pub(crate) struct EventManager{
    ///whether the mouse left button is pushed or not
    mlb : bool,
    hovered_ctl_id : Option<Arc<()>>,
}

impl EventManager{
    pub(crate) fn new() -> EventManager{ EventManager{ mlb : false, hovered_ctl_id : None } }

    pub(crate) fn mouse_move<C : Control + 'static>(&mut self, panel : &mut C, x : usize, y : usize){
        if let Some(id) = find_hoverd_ctl(panel, x, y).map(|id| id.clone()){
            if let Some(prev) = &self.hovered_ctl_id{
                if prev != &id{
                    if let Some(c) = find_ctl_mut(panel, prev){
                        c.on_mouse_leave();
                    }
                    if let Some(c) = find_ctl_mut(panel, &id){
                        c.on_mouse_enter();
                    }
                    self.hovered_ctl_id = Some(id);
                }
            } else{
                if let Some(c) = find_ctl_mut(panel, &id){
                    c.on_mouse_enter();
                }
                self.hovered_ctl_id = Some(id);
            }
        }
    }

    pub(crate) fn mouse_press<C : Control + 'static>(&mut self, panel : &mut C){
        if self.mlb == false {
            self.mlb = true;
            if let Some(id) = &self.hovered_ctl_id {
                if let Some(c) = find_ctl_mut(panel, &id) {
                    c.on_mouse_click();
                }
            }
        }
    }

    pub(crate) fn mouse_release<C : Control + 'static>(&mut self, panel : &mut C){
        self.mlb = false;
    }
}