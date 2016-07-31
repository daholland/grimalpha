#[macro_use] extern crate imgui;
#[macro_use] extern crate glium;

#[macro_use] mod app;

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
pub struct Vertex {
    pos: [f32; 2],
    tex_coords: [f32; 2]
}

implement_vertex!(Vertex, pos, tex_coords);



fn main() {
    let config = util::read_config().unwrap();

    //#[cfg(feature = "xinput")]
    //let mut padstate = input::xinput::JoyPadState::new();

    let mut app = App::init(config);

    
    'outer: loop {
        //#[cfg(feature = "xinput")]
        //unsafe {
        //    let result = input::XInputGetState(0, &mut padstate);
            //println!("result: {:?}, padstate: {:?}", result, padstate);
        //}
        


        for ev in app.display.poll_events() {
            {
                use glium::glutin::{Event, ElementState, MouseButton};
                
                if let Event::Closed = ev {
                    break 'outer
                };

                if let Event::MouseMoved(x, y) = ev {
                    app.input_state.mouse.pos = (x, y)
                };


                if let Event::MouseInput(state, MouseButton::Left) = ev {
                    app.input_state.mouse.buttons.0 = state == ElementState::Pressed
                }

                //println!("mouse (x,y): {:?}", app.input_state.mouse.pos);
            
            }
        }

        app.render((1.0, 1.0, 1.0, 1.0), ui::hello_world);

    }

    println!("steamapi inited: {:?}", app.steam_api);

    //unsafe {
    //    steamworks::SteamAPI_Shutdown();
    //}
}
