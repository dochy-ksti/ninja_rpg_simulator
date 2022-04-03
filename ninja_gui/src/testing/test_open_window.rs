use std::path::Path;
use crate::imp::start_loop::start_loop;
use crate::testing::test_data::test_data;
use crate::testing::test_world::TestWorld;

pub fn test_open_window<P : AsRef<Path>>(font_path : P){
    let mut world = TestWorld::new();
    start_loop(font_path, world.get_ini(), move |a| world.modify_and_get(a));
}