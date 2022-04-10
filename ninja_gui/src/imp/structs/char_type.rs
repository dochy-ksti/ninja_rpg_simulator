
pub(crate) enum CharType{
    English(char),
    CJK(char),
    Open(char),
    Close(char),
    Whitespace(char),
    None,
}

impl CharType{

    // ///rhsのenum variantにし、lhsの文字列の後にrhsの文字列をくっつける
    // pub(crate) fn replace_and_concat(&mut self, rhs : CharType){
    //     let lhs = std::mem::replace(self, rhs);
    //     if let Some(lhs) = lhs.into_string() {
    //         match self {
    //             CharType::English(s) | CharType::CJK(s) |
    //             CharType::Open(s) | CharType::Close(s) |
    //             CharType::Whitespace(s) => s.insert_str(0, &lhs),
    //             CharType::None =>{},
    //         }
    //     }
    // }

    pub(crate) fn get_char(&self) -> Option<char> {
        match self {
            CharType::English(s) | CharType::CJK(s)  |
            CharType::Open(s) | CharType::Close(s) |
            CharType::Whitespace(s) => Some(*s),
            CharType::None => None,
        }
    }

    pub(crate) fn to_open(c : char) -> Option<CharType>{
        match c{
            '(' | '[' | '{' |
            '（' | '「' | '【' |
            '『' | '［' | '〈' |
            '《' | '｛' | '«' |
            '‹' | '〔' | '〘' |
            '〚' | '“' | '‘' => Some(CharType::Open(c)),
            _ => None,
        }
    }

    pub(crate) fn to_close(c : char) -> Option<CharType>{
        match c{
            ')' | ']' | '}' |
            '）' | '」' | '】' |
            '』' | '］' | '〉' |
            '》' | '｝' | '»' |
            '›' | '〕' | '〙' |
            '〛' | '”' | '’' |
            ',' | '.' | '、' |
            '。' | '，' | '．' |
            '‥' | '…' => Some(CharType::Close(c)),
            _ => None,
        }
    }

    pub(crate) fn to_whitespace(c : char) -> Option<CharType>{
        if c.is_whitespace(){
            Some(CharType::Whitespace(c))
        } else{
            None
        }
    }

    pub(crate) fn to_cjk(c : char) -> Option<CharType>{
        if unicode_blocks::is_cjk(c){
            Some(CharType::CJK(c))
        } else{
            None
        }
    }
}