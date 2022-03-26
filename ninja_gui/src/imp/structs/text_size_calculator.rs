use crate::imp::structs::gui_size::GuiSize;

pub(crate) struct TextSizeCalculator{
    x : usize,
    width : usize,
    height : usize,
    line_height : usize,
    max_width : usize,
}

impl TextSizeCalculator{
    pub(crate) fn new(line_height : usize, max_width : usize) -> TextSizeCalculator{
        TextSizeCalculator{ x : 0, width : 0, height : line_height, line_height, max_width }
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
        if self.x < self.width{
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