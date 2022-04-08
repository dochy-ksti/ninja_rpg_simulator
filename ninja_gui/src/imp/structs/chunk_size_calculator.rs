use piston_window::CharacterCache;
use piston_window::types::FontSize;
use crate::imp::structs::gui_size::GuiSize;
use crate::imp::structs::text_chunk::TextChunk;
use crate::PistonGlyph;

//基本的には、英語と、スペースで区切る全言語と、日本語に対応する。韓国語や中国語にも対応できているかもしれないがわからない。
//そもそも日本語の通常の改行法と違うのでヤヴァイ説がある。

// Engligh : 基本的にspaceでしか改行されない。判定法はCJK,Open,Close,Whitespace以外全部。
// CJK : 中国語日本語韓国語。改行可能文字またはスペースでしか改行されない。改行可能文字の連続は改行されない。is_cjkかつsymbol,whitespaceでないもの
// Open 左側改行可能文字 (など。左側がCJKまたは右側改行可能文字なら改行可能。個別指定する。
// CJK_Open 左側改行可能文字で、Openに加えて左側がEnglishであっても改行可能。個別指定。基本的には左側改行可能文字中ASCIIでないものになる
// Close 右側改行可能文字)、。,.など 上を参照
// CJK_Close 上を参照
// WhiteSpace 両端に何が来ても改行可能。Spaceの連続の場合でもどこでも改行可能。
enum CharType{
    English(String),
    CJK(String),
    Open(String),
    CJK_Open(String),
    Close(String),
    CJK_Close(String),
    WhiteSpace(char),
    None,
}

impl CharType{
    pub(crate) fn english(c : char) -> CharType{ CharType::English(c.to_string()) }
    pub(crate) fn cjk(c : char) -> CharType{ CharType::CJK(c.to_string()) }
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

