extern crate glium;
extern crate time;
extern crate winapi;

// TODO: split out into own mod for Support/Systems holder
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin;
use imgui::{ImGui, ImGuiKey, Ui};
use imgui::glium_renderer::Renderer;

use self::time::SteadyTime;

use steamworks::{SteamAPI, SteamAPI_Init};

pub struct App {
    last_frame: SteadyTime,
    pub display: GlutinFacade,
    pub ui_sys: ImGui,
    pub ui_state: (),
    renderer: Renderer,
    shader: glium::Program,
    pub steam_api: Option<SteamAPI>,
    log: (),
    event_sys: (),
    resource_sys: (),
}

struct AppBuilder;

impl App {
    pub fn init() -> App {
        use glium::*;
        let mut steam_api = None;

        unsafe {
            if SteamAPI_Init() {
                steam_api = Some(SteamAPI {loaded: true })
            };
        }


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

void main() {
gl_Position = vec4(pos, 0.0, 1.0);
}
"#;
        let frag_shader_src = r#"
#version 140

out vec4 color;

void main() {
color = vec4(1.0, 0.0, 0.0, 1.0);
}
"#;

            let program = glium::Program::from_source(&display, vert_shader_src, frag_shader_src,None).unwrap();

        App {
            display: display.clone(),
            ui_sys: imgui,
            ui_state: (),
            renderer: renderer,
            last_frame: SteadyTime::now(),
            shader: program,
            steam_api: steam_api,
            log: (),
            event_sys: (),
            resource_sys: ()
             
             
        }
    }
    pub fn render<F:FnMut(&Ui)>(&mut self, clear_color: (f32,f32,f32,f32), mut run_ui: F)
        where F:FnMut(&Ui) {
        use glium::*;
        let now = SteadyTime::now();
        let delta = now - self.last_frame;
        let delta_f = delta.num_nanoseconds().unwrap() as f32/ 1_000_000_000.0;
        self.last_frame = now;

        //begin draw
        let mut target = self.display.draw();
        target.clear_color(clear_color.0, clear_color.1, clear_color.2, clear_color.3);

        let vert1 = ::Vertex { pos: [-0.5, -0.5]};
        let vert2 = ::Vertex { pos: [0.0, 0.5]};
        let vert3 = ::Vertex { pos: [0.5, -0.25]};
        let shape = vec![vert1, vert2, vert3];

        let vert_buffer = glium::VertexBuffer::new(&self.display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        target.draw(&vert_buffer, &indices, &self.shader, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();


        // ui render
        let window = self.display.get_window().unwrap();

        let size_pts = window.get_inner_size_points().unwrap();
        let size_px = window.get_inner_size_pixels().unwrap();

        let ui = self.ui_sys.frame(size_pts, size_px, delta_f);

        run_ui(&ui);

        self.renderer.render(&mut target, ui).unwrap();
        //finish draw call

        target.finish().unwrap();
        
        
    }
}
pub mod ui {
    use imgui::{Ui, ImGuiSetCond_FirstUseEver};
    pub fn hello_world(ui: &Ui) {
        ui.window(im_str!("Hello world!"))
            .size((300.0, 100.0), ImGuiSetCond_FirstUseEver)
            .build(|| {
                ui.text(im_str!("Hello world"));
                ui.text(im_str!("asdfasdf"));
                ui.separator();
                ui.text(im_str!("asdfasdf"));

        });
    }
}


pub mod input {

use super::winapi::*;

    #[link(name = "xinput")]
    extern "system" {
        pub fn XInputEnable(enable: BOOL);

        pub fn XInputGetAudioDeviceIds(
            dwUserIndex: DWORD, pRenderDeviceId: LPWSTR, pRenderCount: *mut UINT,
            pCaptureDeviceId: LPWSTR, pCaptureCount: *mut UINT
        ) -> DWORD;

        pub fn XInputGetBatteryInformation(
            dwUserIndex: DWORD, devType: BYTE, pBatteryInformation: *mut XINPUT_BATTERY_INFORMATION
        ) -> DWORD;

        pub fn XInputGetCapabilities(
            dwUserIndex: DWORD, dwFlags: DWORD, pCapabilities: *mut XINPUT_CAPABILITIES
        ) -> DWORD;

        pub fn XInputGetDSoundAudioDeviceGuids(
            dwUserIndex: DWORD, pDSoundRenderGuid: *mut GUID, pDSoundCaptureGuid: *mut GUID
        ) -> DWORD;

        pub fn XInputGetKeystroke(
            dwUserIndex: DWORD, dwReserved: DWORD, pKeystroke: PXINPUT_KEYSTROKE
        ) -> DWORD;

        pub fn XInputGetState(dwUserIndex: DWORD, pState: *mut XINPUT_STATE) -> DWORD;

        pub fn XInputSetState(dwUserIndex: DWORD, pVibration: *mut XINPUT_VIBRATION) -> DWORD;
    }

    pub struct JoyPadState(XINPUT_STATE);

    impl JoyPadState {
        pub fn new() -> XINPUT_STATE {
            XINPUT_STATE {
                dwPacketNumber: 0,
                Gamepad: XINPUT_GAMEPAD {
                    wButtons: 0,//WORD,
                    bLeftTrigger: 0,// BYTE,
                    bRightTrigger: 0,//BYTE,
                    sThumbLX: 0,//SHORT,
                    sThumbLY: 0,//SHORT,
                    sThumbRX: 0,//SHORT,
                    sThumbRY: 0,//SHORT,
                }
            }
        }
    }
}

