use std::ops::Add;
use crate::imp::structs::gui_size::GuiSize;

#[derive(Copy, Clone, Debug)]
pub(crate) struct GuiPoint{
    x : isize,
    y : isize,
}

impl GuiPoint{
    pub(crate) fn new(x : isize, y : isize) -> GuiPoint{ GuiPoint{ x,y } }
    pub(crate) fn x(&self) -> isize{ self.x }
    pub(crate) fn y(&self) -> isize{ self.y }
    pub(crate) fn rect(&self, size : GuiSize) -> [f64;4]{ [self.x as f64, self.y as f64, size.w() as f64, size.h() as f64] }
}

impl Add for GuiPoint{
    type Output = GuiPoint;

    fn add(self, rhs: Self) -> Self::Output {
        GuiPoint::new(self.x + rhs.x, self.y + rhs.y)
    }
}