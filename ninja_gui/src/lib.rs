#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]

use std::path::Path;


mod imp;
#[allow(dead_code)]
mod testing;

pub use imp::structs::gui_item::{GuiItem, GuiItems};
pub use imp::structs::gui_output::GuiOutput;

pub use crate::imp::start_loop::start_loop;
pub use crate::testing::test_open_window::test_open_window;