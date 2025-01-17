use imgui::Ui;


pub fn show_window(ui: &mut Ui) {
    let size = ui.io().display_size;
    ui.window("Winny")
        .size([size[0], size[1]], imgui::Condition::Always)
        .position([0., 0.], imgui::Condition::Always)
        .resizable(false)
        .collapsible(false)
        .title_bar(false)
        .build(|| {
            ui.text("heyyyy");
        });
    //println!("uh {:?}", ui.window_size());
}
