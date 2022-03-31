use std::path::Path;
use piston_window::{clear, Glyphs, PistonWindow, text, TextureSettings, WindowSettings, Transformed, TextureContext, EventLoop, Event, Loop, Input, Motion, MouseCursorEvent, MouseRelativeEvent, ButtonEvent, Button, MouseButton, ButtonState};
use piston_window::glyph_cache::rusttype::GlyphCache;
use crate::{GuiItems, PistonGlyph};
use crate::imp::control::Control;
use crate::imp::create_panel::create_panel;
use crate::imp::structs::draw_context::DrawContext;
use crate::imp::structs::event_manager::EventManager;
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
    let mut glyph : PistonGlyph = Glyphs::new(font_path, window.create_texture_context(), ts).unwrap();
    let mut panel = create_panel(&gui_items);
    let mut event_manager = EventManager::new();


    while let Some(e) = window.next() {
        e.mouse_cursor(|p| {
            event_manager.mouse_move(
                &mut panel, p[0] as usize, p[1] as usize)
        });
        e.button(|b|{
            match b.button{
                Button::Mouse(MouseButton::Left) =>{
                    match b.state{
                        ButtonState::Press =>{
                            event_manager.mouse_press(&mut panel);
                        },
                        ButtonState::Release =>{
                            event_manager.mouse_release(&mut panel);
                        }
                    }
                },
                _ =>{}
            }
        });

        match e {
            Event::Loop(Loop::Render(_)) => {
                window.draw_2d(&e, |c, g, d| {
                    clear([0.0, 0.0, 0.0, 1.0], g);

                    let mut dc = DrawContext::new( &c, g, &mut glyph,);

                    panel.draw(&mut dc);
                    // text::Text::new_color([1.0, 1.0, 1.0, 1.0], 16).draw(
                    //     "Hello World",
                    //     &mut glyph,
                    //     &c.draw_state,
                    //     c.transform.trans(100., 100.), g
                    // ).unwrap();
                    glyph.factory.encoder.flush(d);
                });
            },
            _ =>{}
        }
    }
}

