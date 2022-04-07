use crate::imp::structs::gui_size::GuiSize;

#[derive(Debug, Clone)]
pub(crate) struct TextChunk{
    text : String,
    size : GuiSize,
}

impl TextChunk{
    pub(crate) fn new(text : String, size : GuiSize) -> TextChunk{ TextChunk{ text, size } }
    pub(crate) fn deconstruct(self) -> (String, GuiSize){ (self.text, self.size) }
}