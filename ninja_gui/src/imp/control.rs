use crate::imp::structs::gui_color::GuiColor;
use crate::imp::structs::gui_point::GuiPoint;
use crate::imp::structs::gui_size::GuiSize;

pub(crate) trait Control{
    fn size(&self) -> GuiSize;
    fn location(&self) -> GuiPoint;
    fn set_location(&mut self, p : GuiPoint);
    fn is_hover(&self) -> bool;
    fn set_hover(&mut self, b : bool);
    fn children(&mut self) -> Option<Box<dyn Iterator<Item=&mut (dyn Control + 'static)> + '_>>;
}