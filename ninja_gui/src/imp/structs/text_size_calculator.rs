use crate::imp::structs::gui_size::GuiSize;
use crate::imp::structs::text_chunk::TextChunk;
use crate::PistonGlyph;

pub(crate) struct TextSizeCalculator{
    x : usize,
    width : usize,
    height : usize,
    line_height : usize,
    line_gap : usize,
    max_width : usize,
    lines : Vec<String>,
}

impl TextSizeCalculator{
    pub(crate) fn new(line_height : usize, line_gap : usize, max_width : usize) -> TextSizeCalculator{
        TextSizeCalculator{ x : 0, width : 0, height : line_height, line_height, line_gap, max_width, lines : vec![String::new()] }
    }

    pub(crate) fn write_chunk(&mut self, chunk : TextChunk){
        let (text, width) = chunk.deconstruct();

        if self.x == 0 || self.x + width < self.max_width{
            self.move_carret(width, text);
        } else{
            self.new_line();
            self.move_carret(width, text);
        }
    }
    
    fn move_carret(&mut self, width : usize, text : String){
        self.x += width;
        if self.width < self.x{
            self.width = usize::min(self.x, self.max_width);
        }
        self.lines.last_mut().unwrap().push_str(&text);
    }

    pub(crate) fn new_line(&mut self){
        self.x = 0;
        self.height += self.line_height + self.line_gap;
        self.lines.push(String::new());
    }
    
    pub(crate) fn get_size(&self) -> GuiSize{
        GuiSize::new(self.width, self.height)
    }

    pub(crate) fn deconstruct(self) -> (Vec<String>, GuiSize){
        let size = self.get_size();
        (self.lines, size)
    }
}