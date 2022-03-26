#[derive(Copy, Clone, Debug)]
pub(crate) struct GuiSize{
    w : usize,
    h : usize,
}

impl GuiSize{
    pub(crate) fn new(w : usize, h : usize) -> GuiSize{ GuiSize{ w, h } }
    pub(crate) fn w(&self) -> usize{ self.w }
    pub(crate) fn h(&self) -> usize{ self.h }
}