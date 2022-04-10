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
    font_size : usize,
    buff : String,
    last : CharType,
}

impl ChunkSizeCalculator{
    pub(crate) fn new(font_size : usize) -> ChunkSizeCalculator{ ChunkSizeCalculator{ font_size, buff : String::new(), last : CharType::None } }
    pub(crate) fn write(&mut self, c : char, glyph : &mut PistonGlyph) -> Option<TextChunk>{
        if let Some(c) = CharType::to_whitespace(c){
            return self.exit( c, glyph);
        }
        else if let Some(c) = CharType::to_open(c){
            match &self.last {
                CharType::CJK(_) | CharType::Whitespace(_) |
                CharType::Close(_) => {
                    return self.exit(c, glyph);
                }
                CharType::None | CharType::English(_) |
                CharType::Open(_) => self.concat(c)
            }
        }
        else if let Some(c) = CharType::to_close(c){
            match &self.last{
                CharType::Whitespace(_) => {
                    return self.exit(c, glyph);
                }
                CharType::English(_) | CharType::CJK(_) |
                CharType::Open(_) | CharType::Close(_) |
                CharType::None => self.concat(c),
            }
        }
        else if let Some(c) = CharType::to_cjk(c){
            match &self.last{
                CharType::CJK(_) | CharType::Close(_) | CharType::Whitespace(_) =>{
                    return self.exit(c, glyph);
                },
                CharType::English(_) | CharType::Open(_) | CharType::None =>{
                    self.concat(c);
                }
            }
        } else{
            match &self.last{
                CharType::Whitespace(_) => {
                    return self.exit( CharType::English(c), glyph);
                }
                CharType::English(_) | CharType::CJK(_) |
                CharType::Open(_) | CharType::Close(_) |
                CharType::None => self.concat(CharType::English(c)),
            }
        }
        return None;
    }

    fn exit(&mut self, new_buff : CharType, glyph : &mut PistonGlyph) -> Option<TextChunk>{
        let buff = std::mem::replace(&mut self.buff,
                                     new_buff.get_char().map_or(String::new(), |c| c.to_string()));
        self.last = new_buff;

        if !buff.is_empty(){
            Some(calc_chunk(buff, self.font_size, glyph))
        } else{
            None
        }
    }

    fn concat(&mut self, c : CharType){
        if let Some(c) = c.get_char() {
            self.buff.push(c);
        }
        self.last = c;
    }

    pub(crate) fn flush(&mut self, glyph : &mut PistonGlyph) -> Option<TextChunk>{
        self.exit(CharType::None, glyph)
    }
}


fn calc_chunk(s : String, font_size : usize, glyph : &mut PistonGlyph) -> TextChunk{
    let width = glyph.width(font_size as u32, &s).unwrap_or(0.);

    TextChunk::new(s, width as usize)
}

pub(crate) fn point_to_pixel(point : usize) -> usize{
    ((point as f32) * 1.333).round() as usize
}

