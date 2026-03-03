use egui::Color32;
use std::sync::OnceLock;

pub static FADED_COLOR: OnceLock<Color32> = OnceLock::new();

pub fn faded_color() -> Color32 {
    FADED_COLOR.get().unwrap_or(&Color32::GRAY).clone()
}

pub fn faded_red_color() -> Color32 {
    Color32::RED.gamma_multiply(0.25)
}
