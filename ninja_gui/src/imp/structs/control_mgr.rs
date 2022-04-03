use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use crate::{GuiItems, GuiOutput};
use crate::imp::control::Control;
use crate::imp::structs::gui_color::GuiColor;
use crate::imp::structs::gui_input::GuiInput;
use crate::imp::structs::root_gui::RootGui;
use crate::imp::structs::text_box::TextBox;
use crate::imp::structs::vert_panel::VertPanel;

pub(crate) struct ControlManager{
    root : Rc<RefCell<RootGui>>,
}

impl ControlManager {
    pub(crate) fn root(&self) -> &Rc<RefCell<RootGui>>{ &self.root }

    pub(crate) fn new<F: FnMut(GuiOutput) -> GuiInput + 'static>(input: &GuiInput, interaction : F) -> ControlManager {
        let panel = VertPanel::new(vec![], GuiColor::BLACK, 2);
        let cmgr = ControlManager { root : Rc::new(RefCell::new(RootGui::Vert(panel))) };

        let root = Self::create_root_gui(cmgr.root.clone(), input, interaction);
        *cmgr.root.as_ref().borrow_mut() = root;

        cmgr
    }

    pub(crate) fn create_root_gui<F : FnMut(GuiOutput) -> GuiInput + 'static>(root : Rc<RefCell<RootGui>>, input : &GuiInput, interaction : F) -> RootGui{
        let interaction = Rc::new(RefCell::new(interaction));
        match input {
            GuiInput::Items(items) => {
                let mut vec: Vec<Box<dyn Control>> = vec![];
                for (index, item) in items.items().iter().enumerate() {
                    let interaction = interaction.clone();
                    let func = move || {
                        let input : GuiInput  = (interaction.as_ref().borrow_mut())(GuiOutput::SelectedIndex(index));
                        let new_root = Self::create_root_gui(root.clone(), &input, interaction.clone());
                        *(root.as_ref().borrow_mut()) = new_root;
                    };
                    let tb = TextBox::new(
                        item.title().to_string(),
                        12,
                        20,
                        20,
                        400,
                        GuiColor::BLACK,
                        GuiColor::WHITE,
                        GuiColor::GRAY,
                        func,
                    );
                    vec.push(Box::new(tb));
                }
                RootGui::Vert(VertPanel::new(vec, GuiColor::BLACK, 2))
            },
        }
    }
}