use piston_window::CharacterCache;
use piston_window::types::FontSize;
use crate::imp::structs::char_type::CharType;
use crate::imp::structs::gui_size::GuiSize;
use crate::imp::structs::text_chunk::TextChunk;
use crate::PistonGlyph;

//基本的には、英語と、スペースで区切る全言語と、日本語に対応する。韓国語や中国語にも対応できているかもしれないがわからない。

// Engligh : 基本的にspaceでしか改行されない。判定法はCJK,Open,Close,Whitespace以外全部。
// CJK : 中国語日本語韓国語。改行可能文字、スペース、CJK同士の間で改行される。EnglishとCJKの間では改行されない。改行可能文字の連続は改行されない。is_cjkかつsymbol,whitespaceでないもの
// Open 左側改行可能文字 (など。左側がCJKまたは右側改行可能文字なら改行可能。個別指定する。
// Close 右側改行可能文字)、。,.など 上を参照
// WhiteSpace 両端に何が来ても改行可能。Spaceの連続の場合でもどこでも改行可能。

pub(crate) struct ChunkSizeCalculator{
    width : usize,
    height : usize,
    font_size : usize,
    buff : CharType,
}

impl ChunkSizeCalculator{
    pub(crate) fn new(font_size : usize) -> ChunkSizeCalculator{ ChunkSizeCalculator{ width : 0, height : 0, font_size, buff : CharType::None } }
    pub(crate) fn write(&mut self, c : char, glyph : &mut PistonGlyph) -> Option<TextChunk>{
        if let Some(c) = CharType::to_whitespace(c){
            return exit(&mut self.buff, c, self.font_size, glyph);
        }
        else if let Some(c) = CharType::to_open(c){
            match &mut self.buff {
                CharType::CJK(_s) | CharType::WhiteSpace(_s) |
                CharType::Close(_s) => {
                    return exit(&mut self.buff, c, self.font_size, glyph);
                }
                CharType::None | CharType::English(_) |
                CharType::Open(_) => self.buff.replace_and_concat(c)
            }
        }
        else if let Some(c) = CharType::to_close(c){
            match &mut self.buff {
                CharType::WhiteSpace(_s) => {
                    return exit(&mut self.buff, c, self.font_size, glyph);
                }
                CharType::English(_) | CharType::CJK(_) |
                CharType::Open(_) | CharType::Close(_) |
                CharType::None => self.buff.replace_and_concat(c),
            }
        }
        else if let Some(c) = CharType::to_cjk(c){
            match &mut self.buff{
                CharType::CJK(_s) | CharType::Close(_s) | CharType::WhiteSpace(_s) =>{
                    return exit(&mut self.buff, c, self.font_size, glyph);
                },
                CharType::English(_) | CharType::Open(_) | CharType::None =>{
                    self.buff.replace_and_concat(c);
                }
            }
        } else{
            match &mut self.buff{
                CharType::WhiteSpace(_s) => {
                    return exit(&mut self.buff, CharType::english(c), self.font_size, glyph);
                }
                CharType::English(_) | CharType::CJK(_) |
                CharType::Open(_) | CharType::Close(_) |
                CharType::None => self.buff.replace_and_concat(CharType::english(c)),
            }
        }
        return None;
    }


}

fn exit(old_buff : &mut CharType, new_buff : CharType, font_size : usize, glyph : &mut PistonGlyph) -> Option<TextChunk> {
    let buff = std::mem::replace(old_buff, new_buff);
    if let Some(s) = buff.into_string(){
        Some(calc_chunk(s, font_size, glyph))
    } else{
        None
    }
}

fn calc_chunk(s : String, font_size : usize, glyph : &mut PistonGlyph) -> TextChunk{
    let width = glyph.width(font_size as u32, &s).unwrap_or(0.);
    let height = point_to_pixel(font_size);
    TextChunk::new(s, GuiSize::new(width as usize, height))
}

pub(crate) fn point_to_pixel(point : usize) -> usize{
    ((point as f32) * 1.333).round() as usize
}

