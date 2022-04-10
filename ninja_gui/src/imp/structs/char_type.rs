
pub(crate) enum CharType{
    English(String),
    CJK(String),
    Open(String),
    Close(String),
    WhiteSpace(String),
    None,
}

impl CharType{
    pub(crate) fn english(c : char) -> CharType{ CharType::English(c.to_string()) }
    pub(crate) fn cjk(c : char) -> CharType{ CharType::CJK(c.to_string()) }
    pub(crate) fn open(c : char) -> CharType{ CharType::Open(c.to_string()) }
    pub(crate) fn close(c : char) -> CharType{ CharType::Close(c.to_string()) }
    pub(crate) fn white_space(c : char) -> CharType{ CharType::WhiteSpace(c.to_string()) }

    ///rhsのenum variantにし、lhsの文字列の後にrhsの文字列をくっつける
    pub(crate) fn replace_and_concat(&mut self, rhs : CharType){
        let lhs = std::mem::replace(self, rhs);
        if let Some(lhs) = lhs.into_string() {
            match self {
                CharType::English(s) | CharType::CJK(s) |
                CharType::Open(s) | CharType::Close(s) |
                CharType::WhiteSpace(s) => s.insert_str(0, &lhs),
                CharType::None =>{},
            }
        }
    }

    pub(crate) fn into_string(self) -> Option<String> {
        match self {
            CharType::English(s) | CharType::CJK(s)  |
            CharType::Open(s) | CharType::Close(s) |
            CharType::WhiteSpace(s) => Some(s),
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
            '〚' | '“' | '‘' => Some(CharType::open(c)),
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
            '‥' | '…' => Some(CharType::close(c)),
            _ => None,
        }
    }

    pub(crate) fn to_whitespace(c : char) -> Option<CharType>{
        if c.is_whitespace(){
            Some(CharType::white_space(c))
        } else{
            None
        }
    }

    pub(crate) fn to_cjk(c : char) -> Option<CharType>{
        if unicode_blocks::is_cjk(c){
            Some(CharType::cjk(c))
        } else{
            None
        }
    }
}