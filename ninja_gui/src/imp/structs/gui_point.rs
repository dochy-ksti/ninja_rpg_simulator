#[derive(Copy, Clone, Debug)]
pub(crate) struct GuiPoint{
    x : isize,
    y : isize,
}

impl GuiPoint{
    pub(crate) fn new(x : isize, y : isize) -> GuiPoint{ GuiPoint{ x,y } }
    pub(crate) fn x(&self) -> isize{ self.x }
    pub(crate) fn y(&self) -> isize{ self.y }
}