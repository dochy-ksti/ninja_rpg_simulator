use crate::{GuiOutput};
use crate::imp::structs::gui_input::GuiInput;
use crate::testing::test_data::test_data;

pub(crate) struct TestWorld{

}

impl TestWorld{
    pub(crate) fn new() -> TestWorld{ TestWorld{} }

    pub(crate) fn get_ini(&self) -> GuiInput {
        GuiInput::Text(test_data())
    }

    pub(crate) fn modify_and_get(&mut self, _out : GuiOutput) -> GuiInput{
        GuiInput::Text(test_data())
    }
}