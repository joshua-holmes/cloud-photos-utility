// TODO: support more styles

/// Published by `Trippasch` on
/// <https://github.com/ocornut/imgui/issues/707>.
pub mod dracula {
    /// Patches the passed style.
    pub fn style_patch(style: &mut imgui::Style) {
        let colors = &mut style.colors;

        colors[imgui::StyleColor::WindowBg as usize] = [0.1f32, 0.1f32, 0.13f32, 1.0f32];
        colors[imgui::StyleColor::MenuBarBg as usize] = [0.16f32, 0.16f32, 0.21f32, 1.0f32];

        // Border
        colors[imgui::StyleColor::Border as usize] = [0.44f32, 0.37f32, 0.61f32, 0.29f32];
        colors[imgui::StyleColor::BorderShadow as usize] = [0.0f32, 0.0f32, 0.0f32, 0.24f32];

        // Text
        colors[imgui::StyleColor::Text as usize] = [1.0f32, 1.0f32, 1.0f32, 1.0f32];
        colors[imgui::StyleColor::TextDisabled as usize] = [0.5f32, 0.5f32, 0.5f32, 1.0f32];

        // Headers
        colors[imgui::StyleColor::Header as usize] = [0.13f32, 0.13f32, 0.17, 1.0f32];
        colors[imgui::StyleColor::HeaderHovered as usize] = [0.19f32, 0.2f32, 0.25f32, 1.0f32];
        colors[imgui::StyleColor::HeaderActive as usize] = [0.16f32, 0.16f32, 0.21f32, 1.0f32];

        // Buttons
        colors[imgui::StyleColor::Button as usize] = [0.44f32, 0.37f32, 0.61f32, 0.54f32];
        colors[imgui::StyleColor::ButtonHovered as usize] = [0.19f32, 0.2f32, 0.25f32, 1.0f32];
        colors[imgui::StyleColor::ButtonActive as usize] = [0.16f32, 0.16f32, 0.21f32, 1.0f32];
        colors[imgui::StyleColor::CheckMark as usize] = [0.74f32, 0.58f32, 0.98f32, 1.0f32];

        // Popups
        colors[imgui::StyleColor::PopupBg as usize] = [0.1f32, 0.1f32, 0.13f32, 0.92f32];

        // Slider
        colors[imgui::StyleColor::SliderGrab as usize] = [0.44f32, 0.37f32, 0.61f32, 0.54f32];
        colors[imgui::StyleColor::SliderGrabActive as usize] = [0.74f32, 0.58f32, 0.98f32, 0.54f32];

        // Frame BG
        colors[imgui::StyleColor::FrameBg as usize] = [0.13f32, 0.13, 0.17, 1.0f32];
        colors[imgui::StyleColor::FrameBgHovered as usize] = [0.19f32, 0.2f32, 0.25f32, 1.0f32];
        colors[imgui::StyleColor::FrameBgActive as usize] = [0.16f32, 0.16f32, 0.21f32, 1.0f32];

        // Tabs
        colors[imgui::StyleColor::Tab as usize] = [0.16f32, 0.16f32, 0.21f32, 1.0f32];
        colors[imgui::StyleColor::TabHovered as usize] = [0.24, 0.24f32, 0.32f32, 1.0f32];
        colors[imgui::StyleColor::TabActive as usize] = [0.2f32, 0.22f32, 0.27f32, 1.0f32];
        colors[imgui::StyleColor::TabUnfocused as usize] = [0.16f32, 0.16f32, 0.21f32, 1.0f32];
        colors[imgui::StyleColor::TabUnfocusedActive as usize] =
            [0.16f32, 0.16f32, 0.21f32, 1.0f32];

        // Title
        colors[imgui::StyleColor::TitleBg as usize] = [0.16f32, 0.16f32, 0.21f32, 1.0f32];
        colors[imgui::StyleColor::TitleBgActive as usize] = [0.16f32, 0.16f32, 0.21f32, 1.0f32];
        colors[imgui::StyleColor::TitleBgCollapsed as usize] = [0.16f32, 0.16f32, 0.21f32, 1.0f32];

        // Scrollbar
        colors[imgui::StyleColor::ScrollbarBg as usize] = [0.1f32, 0.1f32, 0.13f32, 1.0f32];
        colors[imgui::StyleColor::ScrollbarGrab as usize] = [0.16f32, 0.16f32, 0.21f32, 1.0f32];
        colors[imgui::StyleColor::ScrollbarGrabHovered as usize] =
            [0.19f32, 0.2f32, 0.25f32, 1.0f32];
        colors[imgui::StyleColor::ScrollbarGrabActive as usize] =
            [0.24f32, 0.24f32, 0.32f32, 1.0f32];

        // Seperator
        colors[imgui::StyleColor::Separator as usize] = [0.44f32, 0.37f32, 0.61f32, 1.0f32];
        colors[imgui::StyleColor::SeparatorHovered as usize] = [0.74f32, 0.58f32, 0.98f32, 1.0f32];
        colors[imgui::StyleColor::SeparatorActive as usize] = [0.84f32, 0.58f32, 1.0f32, 1.0f32];

        // Resize Grip
        colors[imgui::StyleColor::ResizeGrip as usize] = [0.44f32, 0.37f32, 0.61f32, 0.29f32];
        colors[imgui::StyleColor::ResizeGripHovered as usize] =
            [0.74f32, 0.58f32, 0.98f32, 0.29f32];
        colors[imgui::StyleColor::ResizeGripActive as usize] = [0.84f32, 0.58f32, 1.0f32, 0.29f32];

        style.tab_rounding = 4.0f32;
        style.scrollbar_rounding = 9.0f32;
        style.window_rounding = 7.0f32;
        style.grab_rounding = 3.0f32;
        style.frame_rounding = 3.0f32;
        style.popup_rounding = 4.0f32;
        style.child_rounding = 4.0f32;
    }
}
