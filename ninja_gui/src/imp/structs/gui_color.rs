#[derive(Clone,Copy,Debug)]
pub(crate) struct GuiColor{
    r : u8,
    g : u8,
    b : u8,
    a : u8,
}

impl GuiColor{
    pub(crate) fn r(&self) -> u8{ self.r }
    pub(crate) fn g(&self) -> u8{ self.g }
    pub(crate) fn b(&self) -> u8{ self.b }
    pub(crate) fn a(&self) -> u8{ self.a }
}