use crate::imp::structs::gui_size::GuiSize;
use crate::PistonGlyph;

pub(crate) struct TextSizeCalculator{
    x : usize,
    width : usize,
    height : usize,
    max_width : usize,
}

impl TextSizeCalculator{
    pub(crate) fn new(max_width : usize) -> TextSizeCalculator{
        TextSizeCalculator{ x : 0, width : 0, height : 0, max_width }
    }

    pub(crate) fn write_chunk(&mut self, chunk_size : GuiSize){
        let width = chunk_size.w();
        let height = chunk_size.h();
        if self.x == 0 && self.height == 0{
            self.height = height;
        }
        if self.x == 0 || self.x + width < self.max_width{
            self.move_carret(width);
        } else{
            self.new_line(height);
            self.move_carret(width);
        }
    }
    
    fn move_carret(&mut self, width : usize){
        self.x += width;
        if self.width < self.x{
            self.width = usize::min(self.x, self.max_width);
        }
    }

    pub(crate) fn new_line(&mut self, line_height : usize){
        self.x = 0;
        self.height += line_height;
    }
    
    pub(crate) fn get_size(&self) -> GuiSize{
        GuiSize::new(self.width, self.height)
    }
}