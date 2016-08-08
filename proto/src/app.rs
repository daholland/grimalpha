extern crate glium;
extern crate time;
extern crate imgui_sys;
extern crate cgmath;

// TODO: split out into own mod for Support/Systems holder
use glium::backend::glutin_backend::GlutinFacade;
use imgui::{ImGui, ImGuiKey, Ui};
use imgui::glium_renderer::Renderer;
use self::time::SteadyTime;


use steamworks::SteamAPI;

use ::ui;
use ::input;
use ::util::config;
use ::resource;

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
    state: f32,
}

impl App {
    pub fn init(cfg: config::Config) -> App {
        use glium::*;
        use resource::Resource;
        let (win_res_x, win_res_y) = cfg.video_config.get_resolution();
        println!("x: {:?}, y: {:?}", win_res_x as u32, win_res_y as u32);
        let display = glutin::WindowBuilder::new()
            .with_dimensions(win_res_x as u32, win_res_y as u32)
            .build_glium()
            .unwrap();


        let mut imgui = ImGui::init();
        let renderer = Renderer::init(&mut imgui, &display).unwrap();

        imgui.set_imgui_key(ImGuiKey::A, 0);

        let mut resource_sys =
            resource::ResourceManager { textures: resource::texture::TextureCache::new() };

        {
            let mut path = PathBuf::from(::util::get_curr_dir().unwrap());
            path.push("man.png");

            let mut texture = resource::texture::Texture::new(&display, "man", path, (64, 128));
            let tex_id = resource::make_resource_id(resource::ResourceNs::Texture, "man");

            texture.load(&display).unwrap();

            


            println!("man size: {:?}\ndims: {:?}\nusize size: {:?}",
                     texture.size(),
                     texture.dimensions(),
                     ::std::mem::size_of::<usize>());
            let _ = resource_sys.textures.add(tex_id, texture);
        }

        {
            let mut path = PathBuf::from(::util::get_curr_dir().unwrap());
            path.push("4x4.png");

            let mut texture = resource::texture::Texture::new(&display, "4x4", path, (128, 128));
            let tex_id = resource::make_resource_id(resource::ResourceNs::Texture, "4x4");

            texture.load(&display).unwrap();

            println!("man size: {:?}\ndims: {:?}\nusize size: {:?}",
                     texture.size(),
                     texture.dimensions(),
                     ::std::mem::size_of::<usize>());

            let _ = resource_sys.textures.add(tex_id, texture);
        } 

        // TODO: load from resource manager and change vis for Texture::load


        let vert_shader_src = r#"
#version 150

in vec3 position;
in vec2 tex_coords;
out vec2 v_tex_coords;

uniform mat4 matrix;

void main() {

v_tex_coords = tex_coords;

gl_Position = matrix * vec4(position, 1.0);

}
"#;
        let frag_shader_src = r#"
#version 150

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;


void main() {
  color = texture(tex, v_tex_coords);



}
"#;

        let program = glium::Program::from_source(&display, vert_shader_src, frag_shader_src, None)
            .unwrap();

        App {
            display: display.clone(),
            ui_sys: imgui,
            ui_state: ui::UiState {
                colorpick: [0.0, 0.0, 0.0, 0.0],
                show_color_window: true,
            },
            renderer: renderer,
            last_frame: SteadyTime::now(),
            shader: program,
            steam_api: None,
            log: (),
            event_sys: (),
            resource_sys: resource_sys,
            game: Game { state: -0.5 },
            input_state: input::InputState {
                mouse: input::MouseState {
                    pos: (0, 0),
                    buttons: (false, false, false),
                },
            },
        }
    }
    pub fn update_mouse(&mut self) {
        let scale = self.ui_sys.display_framebuffer_scale();
        self.ui_sys.set_mouse_pos(self.input_state.mouse.pos.0 as f32 / scale.0,
                                  self.input_state.mouse.pos.1 as f32 / scale.1);
        self.ui_sys.set_mouse_down(&[self.input_state.mouse.buttons.0,
                                     self.input_state.mouse.buttons.1,
                                     self.input_state.mouse.buttons.2,
                                     false,
                                     false]);
    }

    pub fn render<F: FnMut(&Ui, &mut ui::UiState)>(&mut self,
                                                   clear_color: (f32, f32, f32, f32),
                                                   mut run_ui: F)
        where F: FnMut(&Ui, &mut ui::UiState)
    {
        use cgmath::prelude::*;
        use glium::*;
        use glium::uniforms::*;


        let now = SteadyTime::now();
        let delta = now - self.last_frame;
        let delta_f = delta.num_nanoseconds().unwrap() as f32 / 1_000_000_000.0;
        self.last_frame = now;
        self.update_mouse();


       
        ////////////////
        // DRAW
        ////////////////
 
        let mut target = self.display.draw();
        let (vp_x, vp_y) = target.get_dimensions();
        println!("Viewport: {:?}, {:?}", vp_x, vp_y);
        target.clear_color_and_depth(clear_color, 1.0);
        
        let shape = glium::vertex::VertexBuffer::new(&self.display, &[
            ::SpriteVertex { position: [0.0,  0.0, 0.0], tex_coords: [0.0, 0.0] },
            ::SpriteVertex { position: [1.0,  0.0, 0.0], tex_coords: [1.0, 0.0] },
            ::SpriteVertex { position: [0.0, 1.0, 0.0], tex_coords: [0.0, 1.0] },
            ::SpriteVertex { position: [1.0,  1.0, 0.0], tex_coords: [1.0, 1.0] },

        ]).unwrap();

        let texture = self.resource_sys.get_raw_texture("man").unwrap();
        let landtexture = self.resource_sys.get_raw_texture("4x4").unwrap();
        let (tex_x, tex_y) = (landtexture.get_width(), texture.get_height().unwrap());
        let landtexture = landtexture;
        println!("Tex_w, tex_h: {:?}, {:?}", tex_x, tex_y);

        let translate = cgmath::Matrix4::from_translation((-1.0, -1.0, 0.0f32).into());
        let scale = cgmath::Matrix4::from_nonuniform_scale(1.0f32/(vp_x as f32/tex_x as f32), 1.0f32/(vp_y as f32/tex_y as f32), 1.0f32);

        let world2view = cgmath::Matrix4::look_at((0.0, 0.0, 1.0f32).into(),
                                      (0.0, 0.0, 0.0f32).into(),
                                      [0.0, 1.0, 0.0f32].into());
        

        let model2world = translate.concat(&scale);

        let view2projection = cgmath::ortho::<f32>(0.0, 960.0, 0.0, 540.0, -1.0, 1.0f32);

        let combined = world2view.concat(&model2world);

        let combined: [[f32; 4]; 4] = combined.into();

        let params = glium::DrawParameters {
            // polygon_mode: PolygonMode::Line,
            dithering: true,
                        .. Default::default()
        };
        let uniforms = uniform! {
            // matrix: [[1.0,0.0,0.0,0.0], [0.0,1.0,0.0,0.0], [0.0,0.0,1.0,0.0], [0.0,0.0,0.0,1.0f32],],
            matrix: combined,
            tex: landtexture
        };
        target.draw(&shape,
                    glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                  &self.shader,
                  &uniforms,
                  &params)
            .unwrap();

        ////////////////
        // UI
        ////////////////

        let window = self.display.get_window().unwrap();

        let size_pts = window.get_inner_size_points().unwrap();
        let size_px = window.get_inner_size_pixels().unwrap();

        let ui = self.ui_sys.frame(size_pts, size_px, delta_f);

        run_ui(&ui, &mut self.ui_state);

        self.renderer.render(&mut target, ui).unwrap();
        // finish draw call

        target.finish().unwrap();

    }
}
