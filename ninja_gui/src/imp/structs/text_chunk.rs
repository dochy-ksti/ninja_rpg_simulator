use crate::imp::structs::gui_size::GuiSize;

#[derive(Debug, Clone)]
pub(crate) struct TextChunk{
    text : String,
    width : usize,
}

impl TextChunk{
    pub(crate) fn new(text : String, width : usize) -> TextChunk{ TextChunk{ text, width } }
    pub(crate) fn deconstruct(self) -> (String, usize){ (self.text, self.width) }
}