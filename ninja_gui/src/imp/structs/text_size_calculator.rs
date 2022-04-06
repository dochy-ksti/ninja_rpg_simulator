use crate::imp::structs::gui_size::GuiSize;
use crate::PistonGlyph;

pub(crate) struct TextSizeCalculator<'a>{
    x : usize,
    width : usize,
    height : usize,
    max_width : usize,
    font_size : usize,
    glyph : &'a PistonGlyph<'a>
}

impl<'a> TextSizeCalculator<'a>{
    pub(crate) fn new(max_width : usize, font_size : usize, glyph : &'a PistonGlyph<'a>) -> TextSizeCalculator{
        TextSizeCalculator{ x : 0, width : 0, height : 0, max_width, font_size, glyph }
    }

    pub(crate) fn write(&mut self, width : usize){
        if self.x == 0 || self.x + width < self.max_width{
            self.move_carret(width);
        } else{
            self.new_line();
            self.move_carret(width);
        }
    }
    
    fn move_carret(&mut self, width : usize){
        self.x += width;
        if self.width < self.x{
            self.width = self.x;
        }
    }

    pub(crate) fn new_line(&mut self){
        self.x = 0;
        self.height += self.line_height;
    }
    
    pub(crate) fn get_size(&self) -> GuiSize{
        GuiSize::new(self.width, self.height)
    }
}