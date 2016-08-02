use imgui::{Ui, ImGuiSetCond_FirstUseEver, ImVec2, ImStr, PlotLines};

#[derive(Clone, Copy, Debug, Default)]
pub struct UiState {
    pub show_color_window: bool,
    pub colorpick: [f32; 4],
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


        });
}
