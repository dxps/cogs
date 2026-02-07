use crate::constants::{POPUP_ROW_HEIGHT, POPUP_ROW_WIDTH};

pub fn menu_row(ui: &mut egui::Ui, text: &str, width: Option<f32>) -> egui::Response {
    //
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(width.unwrap_or(POPUP_ROW_WIDTH), POPUP_ROW_HEIGHT),
        egui::Sense::click(),
    );

    if response.is_pointer_button_down_on() {
        ui.painter().rect_filled(rect, 4.0, egui::Color32::from_rgb(70, 70, 90));
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 4.0, egui::Color32::from_rgb(55, 55, 70));
    }

    let text_pos = egui::pos2(rect.left() + 8.0, rect.center().y);
    ui.painter().text(
        text_pos,
        egui::Align2::LEFT_CENTER,
        text,
        egui::TextStyle::Button.resolve(ui.style()),
        ui.visuals().text_color(),
    );

    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}
