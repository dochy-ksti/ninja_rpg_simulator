use std::sync::Arc;
use crate::imp::structs::draw_context::DrawContext;
use crate::imp::structs::gui_color::GuiColor;
use crate::imp::structs::gui_point::GuiPoint;
use crate::imp::structs::gui_size::GuiSize;

pub(crate) trait Control{
    fn id(&self) -> &Arc<()>;
    fn size(&self) -> GuiSize;
    fn location(&self) -> GuiPoint;
    fn set_location(&mut self, p : GuiPoint);
    fn on_mouse_leave(&mut self);
    fn on_mouse_enter(&mut self);
    fn on_mouse_click(&mut self);
    fn children(&self) -> Option<Box<dyn Iterator<Item=&(dyn Control + 'static)> + '_>>;
    fn children_mut(&mut self) -> Option<Box<dyn Iterator<Item=&mut (dyn Control + 'static)> + '_>>;

    fn draw(&self, dc : &mut DrawContext);
}