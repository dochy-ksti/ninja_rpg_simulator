use std::path::Path;
use crate::imp::start_loop::start_loop;
use crate::testing::test_data::test_data;

pub fn test_open_window<P : AsRef<Path>>(font_path : P){
    start_loop(font_path, test_data(), &mut |a| test_data());
}