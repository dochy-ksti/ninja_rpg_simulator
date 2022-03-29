#[derive(Clone,Copy,Debug)]
pub(crate) struct GuiColor{
    a : u8,
    r : u8,
    g : u8,
    b : u8,
}

impl GuiColor{
    pub(crate) const fn new(a : u8, r : u8, g : u8, b : u8) -> GuiColor{ GuiColor{ a, r,g,b }}
    pub(crate) fn a(&self) -> u8{ self.a }
    pub(crate) fn r(&self) -> u8{ self.r }
    pub(crate) fn g(&self) -> u8{ self.g }
    pub(crate) fn b(&self) -> u8{ self.b }

    pub(crate) const BLACK : GuiColor = GuiColor::new(255,0,0,0);
    pub(crate) const WHITE : GuiColor = GuiColor::new(255,255,255,255);
}