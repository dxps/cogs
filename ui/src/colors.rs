use egui::Color32;

pub fn faded_color(ui: &egui::Ui) -> Color32 {
    ui.visuals().text_color().gamma_multiply(0.6)
}

pub fn faded_red_color() -> Color32 {
    Color32::RED.gamma_multiply(0.25)
}
