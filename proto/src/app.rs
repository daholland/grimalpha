extern crate glium;
extern crate time;
extern crate imgui_sys;

// TODO: split out into own mod for Support/Systems holder
use glium::backend::glutin_backend::GlutinFacade;
use imgui::{ImGui, ImGuiKey, Ui};
use imgui::glium_renderer::Renderer;

use self::time::SteadyTime;


use steamworks::{SteamAPI};

use ::ui as ui;
use ::input as input;

pub struct App {
    last_frame: SteadyTime,
    pub display: GlutinFacade,
    pub ui_sys: ImGui,
    pub ui_state: ui::UiState,
    renderer: Renderer,
    shader: glium::Program,
    pub steam_api: Option<SteamAPI>,
    log: (),
    event_sys: (),
    resource_sys: (),
    pub game: Game,
    pub input_state: input::InputState,
}

use std::path::Path;

struct AppConfig<'ac> {
    root_path: Option<&'ac str>,
    user_home_dir: Option<&'ac str>,
    resource_path: Option<&'ac str>,
    game_config: Option<&'ac str>
}

impl<'ac> AppConfig<'ac> {
    
}

struct AppBuilder<'app> {
    pub config: AppConfig<'app>,

}

impl<'app> AppBuilder<'app> {
    pub fn new(config_path: Option<&'app str>) -> Self {
        match config_path {
            Some(path) => {
                println!("Path found! {}", path);
                AppBuilder {
                    config: AppConfig {
                        root_path: Some(path),
                        user_home_dir: None,
                        resource_path: None,
                        game_config: None
                    }
                }
            },
            None => {
                println!("No Path given!");
                panic!();
            }
        }
    }

    
}

pub struct Game {
state: f32
}

impl App {
    pub fn init() -> App {
        use glium::*;

        let display = glutin::WindowBuilder::new()
            .with_dimensions(960,540)
            .build_glium().
            unwrap();

        
        let mut imgui = ImGui::init();
        let renderer = Renderer::init(&mut imgui, &display).unwrap();

        imgui.set_imgui_key(ImGuiKey::A, 0);


        let vert_shader_src = r#"
#version 140
in vec2 pos;
out vec2 my_attr;

uniform mat4 matrix;

void main() {
  my_attr = pos;
  gl_Position = matrix * vec4(pos, 0.0, 1.0);
}
"#;
        let frag_shader_src = r#"
#version 140
in vec2 my_attr;
out vec4 color;

void main() {
  color = vec4(my_attr, 0.0, 1.0);
}
"#;

            let program = glium::Program::from_source(&display, vert_shader_src, frag_shader_src,None).unwrap();

        App {
            display: display.clone(),
            ui_sys: imgui,
            ui_state: ui::UiState {
                colorpick: [0.0,0.0,0.0,0.0],
                show_color_window: true
            },
            renderer: renderer,
            last_frame: SteadyTime::now(),
            shader: program,
            steam_api: None,
            log: (),
            event_sys: (),
            resource_sys: (),
            game: Game {
                state: -0.5
            },
            input_state: input::InputState {
                mouse: input::MouseState {
                    pos: (0,0),
                    buttons: (false, false, false),
                }
            }
        }
    }
    pub fn update_mouse(&mut self) {
        let scale = self.ui_sys.display_framebuffer_scale();
        self.ui_sys.set_mouse_pos(self.input_state.mouse.pos.0 as f32 / scale.0,
                                  self.input_state.mouse.pos.1 as f32 / scale.1 );
        self.ui_sys.set_mouse_down(&[self.input_state.mouse.buttons.0,
                                     self.input_state.mouse.buttons.1, self.input_state.mouse.buttons.2, false, false]);
    }

    pub fn render<F: FnMut(&Ui, &mut ui::UiState)>(&mut self, clear_color: (f32,f32,f32,f32), mut run_ui: F)
        where F:FnMut(&Ui, &mut ui::UiState) {
        use glium::*;
        let now = SteadyTime::now();
        let delta = now - self.last_frame;
        let delta_f = delta.num_nanoseconds().unwrap() as f32/ 1_000_000_000.0;
        self.last_frame = now;
        self.update_mouse();

        self.game.state += 0.0002;
        if self.game.state > 0.5 {
            self.game.state = -0.5
        }

        //begin draw
        let mut target = self.display.draw();
        target.clear_color(clear_color.0, clear_color.1, clear_color.2, clear_color.3);

        let vert1 = ::Vertex { pos: [-0.5, -0.5]};
        let vert2 = ::Vertex { pos: [0.0, 0.5]};
        let vert3 = ::Vertex { pos: [0.5, -0.25]};
        let shape = vec![vert1, vert2, vert3];

        let vert_buffer = glium::VertexBuffer::new(&self.display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let t = self.game.state;
        let uniforms = uniform! {
            matrix: [
                [t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [t, 0.0, 0.0, 1.0f32],
            ]  
        };
        target.draw(&vert_buffer, &indices, &self.shader, &uniforms, &Default::default()).unwrap();


        // ui render
        let window = self.display.get_window().unwrap();

        let size_pts = window.get_inner_size_points().unwrap();
        let size_px = window.get_inner_size_pixels().unwrap();

        let ui = self.ui_sys.frame(size_pts, size_px, delta_f);
        
        run_ui(&ui, &mut self.ui_state);

        self.renderer.render(&mut target, ui).unwrap();
        //finish draw call

        target.finish().unwrap();
        
    }
}



