use crate::imp::calc_text_size::calc_text_size;
use crate::imp::control::Control;
use crate::imp::structs::gui_color::GuiColor;
use crate::imp::structs::gui_rect::GuiRect;
use crate::imp::structs::gui_size::GuiSize;

pub(crate) struct TextBox{
    text : String,
    char_size : usize,
    char_width : usize,
    line_height : usize,
    max_width : usize,
    text_color : GuiColor,
    back_color : GuiColor,
    hover_color : GuiColor,
    size : GuiSize,
}

impl TextBox{
    pub(crate) fn new(text : String,
                      char_size : usize,
                      char_width : usize,
                      line_height : usize,
                      max_width : usize,
                      text_color : GuiColor,
                      back_color : GuiColor,
                      hover_color : GuiColor) -> TextBox{
        let size = calc_text_size(&text, char_width, line_height, max_width);

        TextBox{
            text,
            char_size,
            char_width,
            line_height,
            max_width,
            text_color,
            back_color,
            hover_color,
            size
        }
    }
}

impl Control for TextBox{
    fn layout(&mut self) -> GuiSize {
        self.size
    }

    fn set_rect(&mut self, rect: GuiRect) {
        todo!()
    }

    fn back_color(&self) -> GuiColor {
        todo!()
    }

    fn hover_color(&self) -> GuiColor {
        todo!()
    }
}