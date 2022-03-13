use std::path::Path;
use piston_window::{clear, Glyphs, PistonWindow, text, TextureSettings, WindowSettings, Transformed};
use crate::GuiItems;


pub fn start_loop<P : AsRef<Path>, F : FnMut(usize)>(font_path : P, gui_items : GuiItems, choice : &mut F) {

    let mut window: PistonWindow = WindowSettings::new(
        "test_window",
        [1280, 720]
    )
        .build()
        .unwrap();


    let ts = TextureSettings::new();
    let mut glyphs = Glyphs::new(font_path, window.create_texture_context(), ts).unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, d| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 16).draw(
                "Hello World",
                &mut glyphs,
                &c.draw_state,
                c.transform.trans(100., 100.), g
            ).unwrap();
            glyphs.factory.encoder.flush(d);
        });
    }
}