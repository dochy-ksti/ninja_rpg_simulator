use crate::imp::structs::gui_color::GuiColor;
use crate::imp::structs::gui_rect::GuiRect;

pub(crate) trait Control{
    fn calc_rect(&self) -> GuiRect;
    fn back_color(&self) -> GuiColor;
    fn hover_color(&self) -> GuiColor;
}