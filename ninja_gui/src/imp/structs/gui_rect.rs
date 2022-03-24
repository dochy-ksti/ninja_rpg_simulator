#[derive(Copy, Clone, Debug)]
pub(crate) struct GuiRect{
    x : usize,
    y : usize,
    w : usize,
    h : usize,
}

impl GuiRect{
    pub(crate) fn x(&self) -> usize {
        self.x
    }

    pub(crate) fn y(&self) -> usize {
        self.y
    }

    pub(crate) fn w(&self) -> usize {
        self.w
    }

    pub(crate) fn h(&self) -> usize {
        self.h
    }
}