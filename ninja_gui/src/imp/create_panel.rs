use crate::GuiItems;
use crate::imp::control::Control;
use crate::imp::structs::gui_color::GuiColor;
use crate::imp::structs::text_box::TextBox;
use crate::imp::structs::vert_panel::VertPanel;

pub(crate) fn create_panel(items : &GuiItems) -> VertPanel{
    let mut vec : Vec<Box<dyn Control>> = vec![];
    for item in items.items(){
        let tb = TextBox::new(
            item.title().to_string(),
            12,
            20,
            20,
            400,
            GuiColor::BLACK,
            GuiColor::WHITE,
            GuiColor::GRAY,
        );
        vec.push(Box::new(tb));
    }
    VertPanel::new(
        vec,
        GuiColor::BLACK,
        1,
    )
}