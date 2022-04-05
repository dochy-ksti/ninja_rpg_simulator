use crate::{GuiOutput, PistonGlyph};
use crate::imp::control::Control;
use crate::imp::structs::gui_color::GuiColor;
use crate::imp::structs::gui_input::GuiInput;
use crate::imp::structs::text_box::TextBox;
use crate::imp::structs::vert_panel::VertPanel;

pub(crate) struct ControlManager<F: FnMut(GuiOutput) -> GuiInput + 'static>{
    root : Box<dyn Control + 'static>,
    interaction : F
}

impl<F: FnMut(GuiOutput) -> GuiInput + 'static> ControlManager<F> {
    pub(crate) fn root(&self) -> &(dyn Control + 'static){ self.root.as_ref() }
    pub(crate) fn root_mut(&mut self) -> &mut (dyn Control + 'static){ self.root.as_mut() }

    pub(crate) fn new(input: GuiInput, interaction : F, glyph : &PistonGlyph) -> ControlManager<F> {
        let root = Self::create_root_ctl(input, glyph);
        ControlManager { root, interaction }
    }

    pub(crate) fn update(&mut self, output : GuiOutput){
        let input = (self.interaction)(output);
        let root = Self::create_root_ctl(input);
        self.root = root;
    }

    pub(crate) fn create_root_ctl(input : GuiInput, glyph : &PistonGlyph) -> Box<dyn Control + 'static>{
        match input {
            GuiInput::Text(items) => {
                let mut vec: Vec<Box<dyn Control>> = vec![];
                for item in items.into_items() {
                    let tb = TextBox::new(
                        item.title().to_string(),
                        24,
                        40,
                        40,
                        400,
                        GuiColor::BLACK,
                        GuiColor::WHITE,
                        GuiColor::GRAY,
                        item,
                    );
                    vec.push(Box::new(tb));
                }
                Box::new(VertPanel::new(vec, GuiColor::BLACK, 2))
            },
        }
    }
}