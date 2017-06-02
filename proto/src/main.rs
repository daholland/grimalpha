#![allow(dead_code, unused_imports)]

#[macro_use]
extern crate imgui;
extern crate imgui_glium_renderer;
#[macro_use]
extern crate glium;

#[macro_use]mod app;

extern crate png;
extern crate image;
extern crate rustc_serialize;
extern crate uuid;
extern crate cgmath;

mod steamworks;
mod util;
mod ui;
mod input;
mod resource;
use app::*;
use std::path::PathBuf;


#[derive(Copy, Clone)]
pub struct SpriteVertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

implement_vertex!(SpriteVertex, position, tex_coords);



fn main() {
    let config = util::read_config().unwrap();

    // #[cfg(feature = "xinput")]
    // let mut padstate = input::xinput::JoyPadState::new();

    let mut app = App::init(config);


    'outer: loop {
        // #[cfg(feature = "xinput")]
        // unsafe {
        //    let result = input::XInputGetState(0, &mut padstate);
        // println!("result: {:?}, padstate: {:?}", result, padstate);
        // }



        for ev in app.display.poll_events() {
            {
                use glium::glutin::{Event, ElementState, MouseButton};

                if let Event::Closed = ev {
                    break 'outer;
                };

                if let Event::MouseMoved(x, y) = ev {
                    app.input_state.mouse.pos = (x, y)
                };


                if let Event::MouseInput(state, MouseButton::Left) = ev {
                    app.input_state.mouse.buttons.0 = state == ElementState::Pressed
                }
                if let Event::MouseInput(state, MouseButton::Right) = ev {
                    app.input_state.mouse.buttons.2 = state == ElementState::Pressed
                }
                if let Event::MouseInput(state, MouseButton::Middle) = ev {
                    app.input_state.mouse.buttons.1 = state == ElementState::Pressed
                }

                if let Event::MouseInput(state, MouseButton::Other(button_idx)) = ev {
                    //TODO: Fix input::MouseState to have 5 bools
                    let extra_button_name = match button_idx {
                        // Back
                        1 => app.input_state.mouse.buttons.3 = state == ElementState::Pressed,
                        // Forward
                        2 => app.input_state.mouse.buttons.4 = state == ElementState::Pressed,
                        _ => (),
                    };
                }

            }
        }

        app.render((0.2, 0.2, 0.2, 1.0), ui::hello_world);

    }

    println!("steamapi inited: {:?}", app.steam_api);

    // unsafe {
    //    steamworks::SteamAPI_Shutdown();
    // }
}
