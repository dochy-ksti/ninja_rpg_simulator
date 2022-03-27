use crate::imp::structs::gui_color::GuiColor;
use crate::imp::structs::gui_point::GuiPoint;
use crate::imp::structs::gui_size::GuiSize;

pub(crate) trait Control{
    fn size(&self) -> GuiSize;
    fn set_location(&mut self, p : GuiPoint);
    fn back_color(&self) -> GuiColor;
    fn hover_color(&self) -> GuiColor;
}