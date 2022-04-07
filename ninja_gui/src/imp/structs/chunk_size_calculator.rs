use piston_window::CharacterCache;
use piston_window::types::FontSize;
use crate::imp::structs::gui_size::GuiSize;
use crate::imp::structs::text_chunk::TextChunk;
use crate::PistonGlyph;

// Other Letter(日本語？) Other Letter同士の間は改行できる Other letterとalphabetの間も開業できる
// alphabet(lower/upper letter) space以外では改行不可。 alphabetとother letterの間は改行可
// その他企業 日本語、日本語　のように、日本語と連続している記号と日本語の間は改行可能。
// だから日本語→記号の間はくっついて、改行不可。記号→日本語は改行可、としてしまおう。
// 英語→記号、記号→英語は改行不可。

enum CharType{
    English(String),
    Japanese(String),
    Symbol(String),
    WhiteSpace(char),
    None,
}

impl CharType{
    pub(crate) fn english(c : char) -> CharType{ CharType::English(c.to_string()) }
    pub(crate) fn japanese(c : char) -> CharType{ CharType::Japanese(c.to_string()) }
    pub(crate) fn white_space(c : char) -> CharType{ CharType::WhiteSpace(c) }
}

pub(crate) struct ChunkSizeCalculator{
    width : usize,
    height : usize,
    font_size : usize,
    buff : CharType
}

impl ChunkSizeCalculator{
    pub(crate) fn new(font_size : usize) -> ChunkSizeCalculator{ ChunkSizeCalculator{ width : 0, height : 0, font_size, buff : CharType::None } }
    pub(crate) fn write(&mut self, c : char, glyph : &mut PistonGlyph) -> Option<TextChunk>{
        if c.is_whitespace(){
            exit(&mut self.buff, CharType::white_space(c), self.font_size, glyph)
        } else if c.is_ascii(){
            if let
        }
        if c.is_ascii_alphabetic(){
            if let CharType::Alpha(s) = &mut self.buff{
                s.push(c);
            } else{

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

