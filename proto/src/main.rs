#[macro_use] extern crate imgui;
#[macro_use] extern crate glium;

#[macro_use] mod app;
mod steamworks;
use app::*;

#[derive(Copy, Clone)]
pub struct Vertex {
    pos: [f32; 2],
}

implement_vertex!(Vertex, pos);



fn main() {
    let mut padstate = input::JoyPadState::new();
    let mut app = App::init();

    'outer: loop {

        unsafe {
            let result = input::XInputGetState(0, &mut padstate);
            println!("result: {:?}, padstate: {:?}", result, padstate);
        }

        for ev in app.display.poll_events() {
            {
                use glium::glutin::Event;

                if let Event::Closed = ev { break 'outer };
            
            }
        }

        app.render((1.0, 1.0, 1.0, 1.0), ui::hello_world);

    }

    println!("steamapi inited: {:?}", app.steam_api);
    unsafe {
        steamworks::SteamAPI_Shutdown();
    }
}
