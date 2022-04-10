use std::sync::Arc;
use crate::imp::calc_text_size::calc_text_size;
use crate::imp::control::Control;
use crate::imp::structs::draw_context::DrawContext;
use crate::imp::structs::gui_color::GuiColor;
use crate::imp::structs::gui_point::GuiPoint;
use crate::imp::structs::gui_size::GuiSize;
use crate::{GuiOutput, PistonGlyph, TextItem};
use crate::imp::structs::chunk_size_calculator::point_to_pixel;
use crate::imp::structs::gui_id::GuiID;

pub(crate) enum Alignment{
    Left, Center, Right
}

pub(crate) struct TextBox{
    id : GuiID,
    text : Vec<String>,
    font_size : usize,
    line_gap : usize,
    alignment : Alignment,
    text_color : GuiColor,
    back_color : GuiColor,
    hover_color : GuiColor,
    size : GuiSize,
    text_size : GuiSize,
    border : usize,
    location : GuiPoint,
    hover : bool,
    item : TextItem,
}

impl TextBox{
    pub(crate) fn construct(text : String, font_size : usize, line_gap : usize,
                            max_width : usize, alignment : Alignment,
                            text_color : GuiColor, back_color : GuiColor, hover_color : GuiColor,
                            border : usize, item : TextItem, glyph : &mut PistonGlyph) -> TextBox{
        let (text, text_size) = calc_text_size(&text, font_size, line_gap, max_width, glyph);

        TextBox{
            id : GuiID::new(),
            text,
            font_size,
            line_gap,
            alignment,
            text_color,
            back_color,
            hover_color,
            size : text_size + GuiSize::new(border * 2, border * 2),
            text_size,
            border,
            location : GuiPoint::new(0,0),
            hover : false,
            item,
        }
    }


}

impl Control for TextBox{
    fn id(&self) -> &GuiID{ &self.id }
    fn size(&self) -> GuiSize {
        self.size
    }

    fn location(&self) -> GuiPoint {
        self.location
    }

    fn set_location(&mut self, p: GuiPoint) {
        self.location = p
    }

    fn on_mouse_leave(&mut self) {
        self.hover = false;
    }

    fn on_mouse_enter(&mut self) {
        self.hover = true;
    }

    fn on_mouse_click(&mut self) -> Option<GuiOutput>{
        Some(GuiOutput::Text(self.item.clone()))
    }

    fn children(&self) -> Option<Box<dyn Iterator<Item=&(dyn Control + 'static)> + '_>> {
        None
    }
    fn children_mut(&mut self) -> Option<Box<dyn Iterator<Item=&mut (dyn Control + 'static)> + '_>> {
        None
    }

    fn draw(&self, dc: &mut DrawContext, rel_loc : GuiPoint) {
        let loc = rel_loc + self.location;
        if self.hover {
            dc.fill_rect(loc, self.size, self.hover_color);
        } else{
            dc.fill_rect(loc, self.size, self.back_color);
        }
        let line_height = point_to_pixel(self.font_size);
        let text_location = loc + text_location(self.size, self.text_size, self.border, &self.alignment);
        let mut y = line_height; //なんでかしらんけど、多分下端を指定するんだろう・・・
        for line in &self.text {
            dc.draw_text(line, text_location + GuiPoint::new(0, y as isize), self.text_color, self.font_size as u32);
            y += line_height + self.line_gap;
        }
    }
}

fn text_location(ctl_size : GuiSize, text_size : GuiSize, border : usize, alignment : &Alignment) -> GuiPoint{
    let text_region = ctl_size - GuiSize::new(border*2, border*2);
    let gap = text_region - text_size;
    //y軸に関しては今のところいじらない
    let x = match alignment{
        Alignment::Left => 0,
        Alignment::Center => gap.w() / 2,
        Alignment::Right => gap.w(),
    };
    GuiPoint::new((border + x) as isize, border as isize)
}