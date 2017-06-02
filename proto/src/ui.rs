use imgui::{Ui, ImGuiSetCond_FirstUseEver, ImVec2, ImStr, PlotLines};
use input;

#[derive(Clone, Copy, Debug, Default)]
pub struct UiState {
    pub show_color_window: bool,
    pub colorpick: [f32; 4],
    pub mouse_state: input::MouseState,
}

pub fn hello_world(ui: &Ui, uistate: &mut UiState) {

    ui.window(im_str!("Hello world!"))
        .movable(true)
        .size((300.0, 100.0), ImGuiSetCond_FirstUseEver)
        .build(|| {
            ui.text(im_str!("Hello world"));
            ui.text(im_str!("asdfasdf"));
            ui.separator();
            ui.text(im_str!("asdfasdf"));

            ui.separator();
            ui.text(im_str!("Test Window! Haha!"));
            if ui.color_edit4(im_str!("Pickcolor"), &mut uistate.colorpick).build() {
                println!("uistate: {:?}", uistate);
            };

            ui.plot_lines(im_str!("plot lines"), &[0.0f32, 0.5, 1.0, 1.5, 2.0]).build();
            ui.separator();
            let imgui_intern_mouse = ui.imgui().mouse_pos();
            ui.text(im_str!("MouseState: uistate = x: {:.1} y: {:.1} | imgui = x: {:.1} y: {:.1}",
                            uistate.mouse_state.pos.0, uistate.mouse_state.pos.1,
                            imgui_intern_mouse.0, imgui_intern_mouse.1
            ));
            ui.text(im_str!("Buttons: L: {:?}, M: {:?}, R: {:?}, B: {:?}, F: {:?}",
                            uistate.mouse_state.buttons.0,
                            uistate.mouse_state.buttons.1,
                            uistate.mouse_state.buttons.2,
                            uistate.mouse_state.buttons.3,
                            uistate.mouse_state.buttons.4,
            ))


        });
}
