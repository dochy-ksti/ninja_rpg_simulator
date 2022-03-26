use crate::imp::structs::chars_iterator::CharsIterator;
use crate::imp::structs::gui_size::GuiSize;
use crate::imp::structs::text_size_calculator::TextSizeCalculator;

pub(crate) fn calc_text_size(s : &str, char_width : usize, line_height : usize, max_width : usize) -> GuiSize{
    let mut text_size_calculator = TextSizeCalculator::new(line_height, max_width);

    let mut chars = if let Some(chars) = CharsIterator::new(s){ chars } else{
        return text_size_calculator.get_size()
    };

    for c in chars {
        match calc_char_width(c, char_width) {
            CW::Char(w) => text_size_calculator.write(w),
            CW::NewLine => text_size_calculator.new_line(),
        }
    }
    return text_size_calculator.get_size();
}

enum CW{ Char(usize), NewLine }
fn calc_char_width(c : char, char_width : usize) -> CW{
    if c == '\n' || c == '\r'{
        CW::NewLine
    }
    else if c.is_ascii(){
        CW::Char(char_width)
    } else {
        //等幅フォントなので、
        CW::Char(char_width * 2)
    }
}