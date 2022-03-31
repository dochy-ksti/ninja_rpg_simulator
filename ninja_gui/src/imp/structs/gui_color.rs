#[derive(Clone,Copy,Debug)]
pub(crate) struct GuiColor{
    r : u8,
    g : u8,
    b : u8,
    a : u8,
}

impl GuiColor{
    pub(crate) const fn new(r : u8, g : u8, b : u8, a : u8) -> GuiColor{ GuiColor{ r,g,b,a }}
    pub(crate) fn r(&self) -> u8{ self.r }
    pub(crate) fn g(&self) -> u8{ self.g }
    pub(crate) fn b(&self) -> u8{ self.b }
    pub(crate) fn a(&self) -> u8{ self.a }
    pub(crate) fn as_f32_array(&self) -> [f32;4]{ [ self.r as f32 / 255., self.g as f32 / 255., self.b as f32 / 255., self.a as f32 / 255.]}

    pub(crate) const BLACK : GuiColor = GuiColor::new(0, 0, 0, 255);
    pub(crate) const WHITE : GuiColor = GuiColor::new(255, 255, 255, 255);
    pub(crate) const RED : GuiColor = GuiColor::new(255, 0, 0, 255);
}