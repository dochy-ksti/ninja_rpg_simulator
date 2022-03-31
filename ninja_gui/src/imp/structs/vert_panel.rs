use std::sync::Arc;
use crate::imp::control::Control;
use crate::imp::structs::draw_context::DrawContext;
use crate::imp::structs::gui_color::GuiColor;
use crate::imp::structs::gui_point::GuiPoint;
use crate::imp::structs::gui_size::GuiSize;

pub(crate) struct VertPanel {
    id : Arc<()>,
    children : Vec<Box<dyn Control>>,
    back_color : GuiColor,
    border : usize,
    location : GuiPoint,
    size : GuiSize,
}

impl Control for VertPanel {
    fn id(&self) -> &Arc<()>{ &self.id }
    fn size(&self) -> GuiSize { self.size }

    fn location(&self) -> GuiPoint {
        self.location
    }

    fn set_location(&mut self, p: GuiPoint) {
        self.location = p;
    }

    fn on_mouse_leave(&mut self) {}
    fn on_mouse_enter(&mut self) {}
    fn on_mouse_click(&mut self) {}

    fn children(&self) -> Option<Box<dyn Iterator<Item=&(dyn Control + 'static)> + '_>> {
        Some(Box::new(self.children.iter().map(|a| a.as_ref())))
    }
    fn children_mut(&mut self) -> Option<Box<dyn Iterator<Item=&mut (dyn Control + 'static)> + '_>> {
        Some(Box::new(self.children.iter_mut().map(|c| c.as_mut())))
        //Some(Box::new(self.children.iter_mut().map(|c| c.as_mut())))
    }

    fn draw(&self, dc: &mut DrawContext, rel_loc : GuiPoint) {
        dc.fill_rect(rel_loc + self.location, self.size, self.back_color);
        for child in &self.children{
            child.as_ref().draw(dc, rel_loc + self.location);
        }
    }
}

impl VertPanel{
    pub(crate) fn new(mut children : Vec<Box<dyn Control>>,
                      back_color : GuiColor,
                      border : usize) -> VertPanel{
        let x = border as isize;
        let mut y = border as isize;
        let mut w = 1;
        for child in &mut children{
            let size = child.size();
            child.set_location(GuiPoint::new(x, y));
            y += (size.h() + border) as isize;
            if w < size.w(){
                w = size.w();
            }
        }

        VertPanel{
            id : Arc::new(()),
            children,
            back_color : GuiColor::RED,
            border,
            location : GuiPoint::new(0,0),
            size : GuiSize::new(w + x as usize * 2, (y + x) as usize),
        }
    }
    pub(crate) fn border(&self) -> usize{ self.border }
}