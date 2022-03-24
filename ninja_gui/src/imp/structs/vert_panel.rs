use crate::imp::control::Control;
use crate::imp::structs::gui_color::GuiColor;
use crate::imp::structs::gui_rect::GuiRect;

pub(crate) struct VertPanel {
    rect : GuiRect,
    children : Vec<Box<dyn Control>>,
    back_color : GuiColor,
    hover_color : GuiColor,
    border : usize,
}

impl Control for VertPanel {
    fn calc_rect(&self) -> GuiRect {
        self.rect
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