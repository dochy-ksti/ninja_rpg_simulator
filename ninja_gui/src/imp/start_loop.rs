use std::path::Path;
use piston_window::{clear, Glyphs, PistonWindow, text, TextureSettings, WindowSettings, Transformed};


pub(crate) fn start_loop<P : AsRef<Path>>(font_path : P) {

    let mut window: PistonWindow = WindowSettings::new(
        "test_window",
        [500, 500]
    )
        .build()
        .unwrap();


    let ts = TextureSettings::new();
    let mut glyphs = Glyphs::new(font_path, window.create_texture_context(), ts).unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, d| {

            // Set a white background
            clear([1.0, 1.0, 1.0, 1.0], g);
            text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32).draw(
                "Hello World",
                &mut glyphs,
                &c.draw_state,
                c.transform.trans(100., 100.), g
            ).unwrap();
            glyphs.factory.encoder.flush(d);
        });
    }
}