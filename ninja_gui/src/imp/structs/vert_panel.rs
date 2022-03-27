use crate::imp::control::Control;
use crate::imp::structs::gui_color::GuiColor;
use crate::imp::structs::gui_point::GuiPoint;
use crate::imp::structs::gui_rect::GuiRect;
use crate::imp::structs::gui_size::GuiSize;

pub(crate) struct VertPanel {
    children : Vec<Box<dyn Control>>,
    back_color : GuiColor,
    hover_color : GuiColor,
    border : usize,
    location : GuiPoint,
    size : GuiSize,
}

impl Control for VertPanel {
    fn size(&self) -> GuiSize { self.size }

    fn set_location(&mut self, p: GuiPoint) {
        self.location = p;
    }

    fn back_color(&self) -> GuiColor {
        self.back_color
    }

    fn hover_color(&self) -> GuiColor {
        self.hover_color
    }
}

impl VertPanel{
    pub(crate) fn new(mut children : Vec<Box<dyn Control>>,
                      back_color : GuiColor,
                      hover_color : GuiColor,
                      border : usize) -> VertPanel{
        let x = border;
        let mut y = border;
        let mut w = 1;
        for child in &mut children{
            let size = child.size();
            child.set_location(GuiPoint::new(x, y));
            y += size.h();
            if w < size.w(){
                w = size.w();
            }
        }

        VertPanel{
            children,
            back_color,
            hover_color,
            border,
            location : GuiPoint::new(0,0),
            size : GuiSize::new(w + x*2, y + x),
        }
    }
    pub(crate) fn border(&self) -> usize{ self.border }
}