use imgui::Ui;


pub fn show_window(ui: &mut Ui) {
    let size = ui.io().display_size;
    let mut checkbox = false;
    let mut slider = 5;
    ui.window("Winny")
        .size([size[0], size[1]], imgui::Condition::Always)
        .position([0., 0.], imgui::Condition::Always)
        .resizable(false)
        .collapsible(false)
        .title_bar(false)
        .build(|| {
            ui.text("heyyyy");
            ui.bullet(); ui.text("bullet");
            ui.bullet(); ui.text("point");
            ui.checkbox("label", &mut checkbox);
            ui.slider("slider", 0, 10, &mut slider);
            ui.button("button");
        });
    //println!("uh {:?}", ui.window_size());
}
