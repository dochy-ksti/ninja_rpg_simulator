use piston_window::CharacterCache;
use piston_window::types::FontSize;
use crate::imp::structs::gui_size::GuiSize;
use crate::imp::structs::text_chunk::TextChunk;
use crate::PistonGlyph;

enum CharType{
    Alpha(String),
    Number(String),
    JapaneseChar(String),
    Symbol(String),
    Other(char),
    None,
}

impl CharType{
    pub(crate) fn alpha(c : char) -> CharType{ CharType::Alpha(c.to_string()) }
    pub(crate) fn number(c : char) -> CharType{ CharType::Number(c.to_string()) }
    pub(crate) fn japanese_char(c : char) -> CharType{ CharType::JapaneseChar(c) }
    pub(crate) fn symbol(c : char) -> CharType{ CharType::Symbol(c.to_string()) }
    pub(crate) fn other(c : char) -> CharType{ CharType::Other(c) }
}

pub(crate) struct ChunkSizeCalculator{
    width : usize,
    height : usize,
    font_size : usize,
    buff : CharType
}

impl ChunkSizeCalculator{
    pub(crate) fn new() -> ChunkSizeCalculator{ ChunkSizeCalculator{ width : 0, height : 0, buff : None } }
    pub(crate) fn write(&mut self, c : char, glyph : &mut PistonGlyph) -> Option<TextChunk>{
        if c.is_ascii_alphabetic(){
            if let CharType::Alpha(s) = &mut self.buff{
                s.push(c);
            } else{
                exit(&mut self.buff, CharType::alpha(c), self.font_size, glyph)
            }
        } else if c.is_numeric(){
            if let CharType::Number(s) = &mut self.buff{
                s.push(c);
            } else{
                exit(&mut self.buff, CharType::number(c), self.font_size, glyph)
            }
        } else if c.is_ascii_graphic()

    }


}

fn exit(old_buff : &mut CharType, new_buff : CharType, font_size : usize, glyph : &mut PistonGlyph) -> Option<TextChunk> {
    let buff = std::mem::replace(old_buff, new_buff);
    match buff {
        CharType::Number(s) | CharType::Alpha(s) => { Some(calc_chunk(s, font_size, glyph)) },
        CharType::JapaneseChar(c) | CharType::Other(c) =>{ Some(calc_chunk(c.to_string(), font_size, glyph)) },
        CharType::None =>{ None }
    }
}

fn calc_chunk(s : String, font_size : usize, glyph : &mut PistonGlyph) -> TextChunk{
    let width = glyph.width(font_size as u32, &s).unwrap_or(0);
    let height = point_to_pixel(font_size);
    TextChunk::new(s, GuiSize::new(width as usize, height))
}

pub(crate) fn point_to_pixel(point : usize) -> usize{
    ((point as f32) * 1.333).round() as usize
}

