use crate::imp::structs::gui_color::GuiColor;
use crate::imp::structs::gui_rect::GuiRect;
use crate::imp::structs::gui_size::GuiSize;

pub(crate) trait Control{
    fn layout(&mut self) -> GuiSize;
    fn set_rect(&mut self, rect : GuiRect);
    fn back_color(&self) -> GuiColor;
    fn hover_color(&self) -> GuiColor;
}