use crate::imp::structs::gui_size::GuiSize;

pub(crate) fn calc_text_size(s : &str, char_width : usize, line_height : usize, max_width : usize) -> GuiSize{

}

enum CW{ Char(usize), NewLine }
fn char_width(c : char, next_c : Option<char>, char_width : usize) -> (CW, bool){
    if c == '\n'{
        return (CW::Char(char_width), true);
    }
    if c == '\r'{
        if let Some(next_c) = next_c{
            if next_c == '\n'{
                return (CW::NewLine, false);
            } else{
                return (CW::NewLine, true);
            }
        } else{
            return (CW::NewLine, true);
        }
    }
    if c.is_ascii(){
        return (CW::Char(char_width), true);
    }
    //等幅フォントなので、
    return (CW::Char(char_width*2), true);
}