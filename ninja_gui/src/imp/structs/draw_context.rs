use piston_window::{Context, G2d, text, Transformed};
use crate::PistonGlyph;

pub(crate) struct DrawContext<'a, 'b : 'a, 'c : 'a>{
    glyph : &'a mut PistonGlyph<'c>,
    context : &'a Context,
    g2d : &'a mut G2d<'b>,
}

impl<'a, 'b : 'a, 'c : 'a> DrawContext<'a, 'b, 'c>{
    pub(crate) fn new<'x, 'y : 'x, 'z : 'x>(context : &'x Context, g2d : &'x mut G2d<'y>, glyph : &'x mut PistonGlyph<'z>) -> DrawContext<'x, 'y, 'z>{
        DrawContext{ glyph, context, g2d }
    }
    pub(crate) fn draw_text(&mut self, text : &str, font_size : u32,){
        text::Text::new_color([1.0, 1.0, 1.0, 1.0], font_size).draw(
            text,
            self.glyph,
            &self.context.draw_state,
            self.context.transform.trans(100., 100.),
             self.g2d,
        ).unwrap();
    }
}