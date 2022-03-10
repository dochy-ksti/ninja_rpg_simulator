use std::fs::File;

fn main() {
    let file = File::open("ninja_gui_test/assets/ume-tmo3.ttf").unwrap();
    ninja_gui::test_window()
}
