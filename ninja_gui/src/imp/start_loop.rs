use std::path::Path;
use piston_window::{clear, Glyphs, PistonWindow, text, TextureSettings, WindowSettings, Transformed, TextureContext, EventLoop, Event, Loop, Input};
use piston_window::glyph_cache::rusttype::GlyphCache;
use crate::GuiItems;
use crate::imp::structs::gui_output::GuiOutput;


pub fn start_loop<P : AsRef<Path>, F : FnMut(GuiOutput) -> GuiItems>(font_path : P, gui_items : GuiItems, choice : &mut F) {

    let mut window: PistonWindow = WindowSettings::new(
        "test_window",
        [1280, 720]
    )
        .build()
        .unwrap();
    window.set_max_fps(60);
    window.set_ups(60);

    let ts = TextureSettings::new();
    let mut glyphs = Glyphs::new(font_path, window.create_texture_context(), ts).unwrap();

    while let Some(e) = window.next() {

        match e {
            Event::Loop(Loop::Render(_)) => {
                //レンダリング
            }
            Event::Loop(Loop::Update(_)) => {
                //アップデート
            }
            Event::Input(i, _) => {
                Input::Move()
                //入力関係
            }
            _ => {}
        }
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

fn write()