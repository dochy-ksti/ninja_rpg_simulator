#[derive(Copy, Clone, Debug)]
pub(crate) struct GuiPoint{
    x : usize,
    y : usize,
}

impl GuiPoint{
    pub(crate) fn new(x : usize, y : usize) -> GuiPoint{ GuiPoint{ x,y } }
    pub(crate) fn x(&self) -> usize{ self.x }
    pub(crate) fn y(&self) -> usize{ self.y }
}