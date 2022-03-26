use std::str::Chars;

///\r\nを\rに変換する、それだけのために大掛かりな・・・
pub(crate) struct CharsIterator<'a>{
    chars : Chars<'a>,
    buffer : char,
    ended : bool,
}

impl<'a> CharsIterator<'a>{
    pub(crate) fn new(s : &str) -> Option<CharsIterator<'_>>{
        let mut chars = s.chars();
        if let Some(c) = chars.next(){
            Some(CharsIterator{ chars, buffer : c, ended : false })
        } else{
            None
        }
    }

    pub(crate) fn next(&mut self) -> Option<char>{
        let c = self.chars.next();
        if let Some(c) = c{
            if c == '\n' && self.buffer == '\r'{
                let c = self.chars.next();
                //\r\nに対応するための特殊処理・・・
                self.next_char(c)
            } else{
                self.next_char(Some(c))
            }
        } else{
            self.next_char(None)
        }
    }

    fn next_char(&mut self, c : Option<char>) -> Option<char>{
        if let Some(c) = c{
            let r = self.buffer;
            self.buffer = c;
            Some(r)
        } else{
            if self.ended{
                None
            } else{
                self.ended = true;
                Some(self.buffer)
            }
        }
    }
}

impl<'a> Iterator for CharsIterator<'a>{
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}