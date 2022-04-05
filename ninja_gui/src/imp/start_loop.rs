use std::path::Path;
use piston_window::{clear, Glyphs, PistonWindow,  TextureSettings, WindowSettings, EventLoop, Event, Loop, MouseCursorEvent, ButtonEvent, Button, MouseButton, ButtonState};
use crate::{ PistonGlyph};
use crate::imp::structs::control_mgr::ControlManager;
use crate::imp::structs::draw_context::DrawContext;
use crate::imp::structs::event_manager::EventManager;
use crate::imp::structs::gui_color::GuiColor;
use crate::imp::structs::gui_input::GuiInput;
use crate::imp::structs::gui_output::GuiOutput;
use crate::imp::structs::gui_point::GuiPoint;


pub fn start_loop<P : AsRef<Path>, F : FnMut(GuiOutput) -> GuiInput + 'static>(font_path : P, input : GuiInput, interaction : F) {

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
    let mut cmgr = ControlManager::new(input, interaction);
    let mut event_manager = EventManager::new();

    while let Some(e) = window.next() {
        e.mouse_cursor(|p| {
            event_manager.mouse_move(
                cmgr.root_mut(), p[0] as usize, p[1] as usize)
        });
        e.button(|b|{
            match b.button{
                Button::Mouse(MouseButton::Left) =>{
                    match b.state{
                        ButtonState::Press =>{
                            if let Some(output) = event_manager.mouse_press(cmgr.root_mut()){
                                cmgr.update(output);
                            }
                        },
                        ButtonState::Release =>{
                            event_manager.mouse_release(cmgr.root_mut());
                        }
                    }
                },
                _ =>{}
            }
        });

        match e {
            Event::Loop(Loop::Render(_)) => {
                window.draw_2d(&e, |c, g, d| {
                    //c.draw_state.blend = Some(Blend::Alpha);
                    clear(GuiColor::WHITE.as_f32_array(), g);

                    let mut dc = DrawContext::new( &c, g, &mut glyph,);

                    cmgr.root_mut().draw(&mut dc, GuiPoint::new(0,0));
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

