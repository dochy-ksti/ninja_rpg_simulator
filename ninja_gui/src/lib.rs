#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]

use std::path::Path;
use crate::imp::start_loop::start_loop;

mod imp;
#[cfg(test)]#[allow(dead_code)]
mod testing;

pub fn test_window<P : AsRef<Path>>(font_path : P){
    start_loop(font_path)
}

pub use imp::gui_item::{GuiItem, GuiItems};
