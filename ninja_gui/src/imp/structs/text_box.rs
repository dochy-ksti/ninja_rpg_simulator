use crate::imp::control::Control;
use crate::imp::structs::gui_color::GuiColor;
use crate::imp::structs::gui_rect::GuiRect;

pub(crate) struct TextBox{
    text : String,
    size : usize,
    char_width : usize,
    line_height : usize,
    text_color : GuiColor,
    back_color : GuiColor,
    hover_color : GuiColor,
}

impl Control for TextBox{
    fn calc_rect(&self) -> GuiRect {
        todo!()
    }

    fn back_color(&self) -> GuiColor {
        todo!()
    }

    fn hover_color(&self) -> GuiColor {
        todo!()
    }
}