use piston_window::{Context, DrawState, G2d, rectangle, Rectangle, text, Transformed};
use crate::imp::structs::gui_color::GuiColor;
use crate::imp::structs::gui_point::GuiPoint;
use crate::imp::structs::gui_size::GuiSize;
use crate::PistonGlyph;

pub(crate) struct DrawContext<'a, 'b, 'c>{
    glyph : &'a mut PistonGlyph<'c>,
    context : &'a Context,
    g2d : &'a mut G2d<'b>,
}

impl<'a, 'b, 'c> DrawContext<'a, 'b, 'c>{
    pub(crate) fn new(context : &'a Context, g2d : &'a mut G2d<'b>, glyph : &'a mut PistonGlyph<'c>) -> DrawContext<'a, 'b, 'c>{
        DrawContext{ glyph, context, g2d }
    }
    pub(crate) fn draw_text(&mut self, text : &str, abs_loc : GuiPoint, color : GuiColor, font_size : u32,){
        eprintln!("{:?}", color.as_f32_array());
        text::Text::new_color(color.as_f32_array(), font_size).draw(
            text,
            self.glyph,
            &DrawState::default(),
            self.context.transform.trans(abs_loc.x() as f64, abs_loc.y() as f64),
             self.g2d,
        ).unwrap();
    }

    pub(crate) fn fill_rect(&mut self, location : GuiPoint, size : GuiSize, color : GuiColor){
        Rectangle::new(color.as_f32_array()).draw(
            location.rect(size), &DrawState::default(),
            self.context.transform.trans(0.,0.), self.g2d);
    }
}