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
    hover : bool,
}

impl Control for VertPanel {
    fn size(&self) -> GuiSize { self.size }

    fn location(&self) -> GuiPoint {
        self.location
    }

    fn set_location(&mut self, p: GuiPoint) {
        self.location = p;
    }

    fn is_hover(&self) -> bool {
        self.hover
    }

    fn set_hover(&mut self, b: bool) {
        self.hover = b;
    }


    fn children(&mut self) -> Option<Box<dyn Iterator<Item=&mut (dyn Control + 'static)> + '_>> {
        Some(Box::new(self.children.iter_mut().map(|c| c.as_mut())))
    }
}

impl VertPanel{
    pub(crate) fn new(mut children : Vec<Box<dyn Control>>,
                      back_color : GuiColor,
                      hover_color : GuiColor,
                      border : usize) -> VertPanel{
        let x = border as isize;
        let mut y = border as isize;
        let mut w = 1;
        for child in &mut children{
            let size = child.size();
            child.set_location(GuiPoint::new(x, y));
            y += size.h() as isize;
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
            size : GuiSize::new(w + x as usize * 2, (y + x) as usize),
            hover : false,
        }
    }
    pub(crate) fn border(&self) -> usize{ self.border }
}