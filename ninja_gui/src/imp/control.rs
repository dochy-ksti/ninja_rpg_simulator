use std::sync::Arc;
use crate::GuiOutput;
use crate::imp::structs::draw_context::DrawContext;
use crate::imp::structs::gui_id::GuiID;
use crate::imp::structs::gui_point::GuiPoint;
use crate::imp::structs::gui_size::GuiSize;

pub(crate) trait Control{
    fn id(&self) -> &GuiID;
    fn size(&self) -> GuiSize;
    fn location(&self) -> GuiPoint;
    fn set_location(&mut self, p : GuiPoint);
    fn set_size(&mut self, s : GuiSize);
    fn on_mouse_leave(&mut self);
    fn on_mouse_enter(&mut self);
    fn on_mouse_click(&mut self) -> Option<GuiOutput>;
    fn children(&self) -> Option<Box<dyn Iterator<Item=&(dyn Control + 'static)> + '_>>;
    fn children_mut(&mut self) -> Option<Box<dyn Iterator<Item=&mut (dyn Control + 'static)> + '_>>;

    fn draw(&self, dc : &mut DrawContext, rel_loc : GuiPoint);
}