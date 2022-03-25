use crate::imp::control::Control;
use crate::imp::structs::gui_color::GuiColor;
use crate::imp::structs::gui_rect::GuiRect;
use crate::imp::structs::gui_size::GuiSize;

pub(crate) struct VertPanel {
    rect : GuiRect,
    children : Vec<Box<dyn Control>>,
    back_color : GuiColor,
    hover_color : GuiColor,
    border : usize,
}

impl Control for VertPanel {
    fn layout(&mut self) -> GuiSize {
        let x = self.border;
        let mut y = self.border;
        let mut w = 1;
        for child in &mut self.children{
            let size = child.layout();
            child.set_rect(GuiRect::new(x, y, size.w(), size.h()));
            y += size.h();
            if w < size.w(){
                w = size.w();
            }
        }
        return GuiSize::new(w + x*2, y + x);
    }

    fn set_rect(&mut self, rect: GuiRect) {
        self.rect = rect;
    }

    fn back_color(&self) -> GuiColor {
        self.back_color
    }

    fn hover_color(&self) -> GuiColor {
        self.hover_color
    }
}

impl VertPanel{
    pub(crate) fn border(&self) -> usize{ self.border }
}