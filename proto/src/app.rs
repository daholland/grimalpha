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
use ::util::config as config;
use ::resource as resource;

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
    pub resource_sys: resource::ResourceManager,
    pub game: Game,
    pub input_state: input::InputState,
}

use std::path::PathBuf;


// struct AppBuilder {
//     pub config: config::AppConfig,
// }

// impl AppBuilder {
//     pub fn new(config_path: PathBuf) -> Self {
//         println!("Path found! {}", config_path.to_str().unwrap());
//         AppBuilder {
//             config: config::AppConfig {
//                 root_path: config_path,
//                 user_home_dir: PathBuf::from(""),
//                 resource_path: PathBuf::from(""),
//                 game_config: PathBuf::from("").
//             }
            
//         }
        
//     }
// }
    

    


pub struct Game {
state: f32
}

impl App {
    pub fn init(cfg: config::Config) -> App {
        use glium::*;

        let (win_res_x, win_res_y) = cfg.video_config.get_resolution();
        println!("x: {:?}, y: {:?}", win_res_x as u32, win_res_y as u32);
        let display = glutin::WindowBuilder::new()
            .with_dimensions(win_res_x as u32, win_res_y as u32)
            .build_glium()
            .unwrap();

        
        let mut imgui = ImGui::init();
        let renderer = Renderer::init(&mut imgui, &display).unwrap();

        imgui.set_imgui_key(ImGuiKey::A, 0);

        let mut resource_sys = resource::ResourceManager {
            textures: resource::TextureCache::new(),
        };

        let mut path = PathBuf::from(::util::get_curr_dir().unwrap());
        path.push("man.png");

        let man_texId = resource_sys.textures.load_image(path).unwrap();

        let vert_shader_src = r#"
#version 140
in vec2 pos;
in vec2 tex_coords;
out vec2 v_tex_coords;

uniform mat4 matrix;

void main() {
  v_tex_coords = tex_coords;
  gl_Position = matrix * vec4(pos, 0.0, 1.0);
}
"#;
        let frag_shader_src = r#"
#version 140
in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;

void main() {
  color = texture(tex, v_tex_coords);
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
            resource_sys: resource_sys,
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

        let image_id = self.resource_sys.textures.get_keys().pop().unwrap();
        let image = self.resource_sys.textures.get(&image_id).unwrap();
        let image_dimensions = image.dimensions();
        let image = image.into_glium_tex();
        let texture = texture::Texture2d::new(&self.display, image).unwrap();
        
        //begin draw
        let mut target = self.display.draw();
        target.clear_color(clear_color.0, clear_color.1, clear_color.2, clear_color.3);

        let vert1 = ::Vertex { pos: [-0.5, -0.5], tex_coords: [0.0, 0.0]};
        let vert2 = ::Vertex { pos: [0.0, 0.5], tex_coords: [0.0, 1.0]};
        let vert3 = ::Vertex { pos: [0.5, -0.25], tex_coords: [1.0, 0.0]};
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
            ],
            tex: &texture
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



