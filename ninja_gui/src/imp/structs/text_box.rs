use crate::imp::calc_text_size::calc_text_size;
use crate::imp::control::Control;
use crate::imp::structs::gui_color::GuiColor;
use crate::imp::structs::gui_point::GuiPoint;
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
    location : GuiPoint,
    hover : bool,
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
            size,
            location : GuiPoint::new(0,0),
            hover : false,
        }
    }
}

impl Control for TextBox{
    fn size(&self) -> GuiSize {
        self.size
    }

    fn location(&self) -> GuiPoint {
        self.location
    }

    fn set_location(&mut self, p: GuiPoint) {
        self.location = p
    }

    fn is_hover(&self) -> bool {
        self.hover
    }

    fn set_hover(&mut self, b: bool) {
        self.hover = b;
    }

    fn children(&mut self) -> Option<Box<dyn Iterator<Item=&mut (dyn Control + 'static)>>> {
        None
    }
}