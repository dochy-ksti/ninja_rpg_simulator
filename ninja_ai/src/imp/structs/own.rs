///All:一度実行されると全員が所有権を持つ。もう実行できない
///AllRepeatable:一度実行されると全員が所有権を持つ。なんどでも実行できる
///One:最初に実行した一人が所有権を持つ。もう実行できない
///OneRepeatable:一人だけ所有権を持つ。その一人は何度でも実行できる
///Unlimited:誰かが所有権を得ても、また別の人は実行でき、実行すれば所有権が得られる
///UnlimitedRepeatable:誰かが所有権を得ても、また別の人は実行でき、実行すれば所有権が得られる。何度でも実行できる
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u32)]
pub enum Own{
    One = 0b10,
    OneRepeatable = 0b11,
    All = 0b100,
    AllRepeatable = 0b101,
    Unlimited = 0b1000,
    UnlimitedRepeatable = 0b1001,
}

impl Own{
    pub fn repeatable(&self) -> bool{
        *self as u32 % 2 == 1
    }

    pub fn one(&self) -> bool{
        *self as u32 & 0b10 != 0
    }
    pub fn all(&self) -> bool{
        *self as u32 & 0b100 != 0
    }
    pub fn unlimited(&self) -> bool{
        *self as u32 & 0b1000 != 0
    }
}