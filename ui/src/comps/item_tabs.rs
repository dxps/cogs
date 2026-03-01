use egui::{Button, Color32, CursorIcon, RichText};
use serde::{Deserialize, Serialize};

use crate::constants::CORNER_RADIUS;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttrsLinksTab {
    Attributes,
    Links,
}

pub fn horiz_tab(ui: &mut egui::Ui, text: &str, selected: bool) -> egui::Response {
    // Copy needed colors first (no long-lived borrow of `ui`)
    let text_color = ui.visuals().text_color();
    let hover_bg = ui.visuals().widgets.hovered.weak_bg_fill;

    let selected_fg = text_color;
    let unselected_fg = text_color.gamma_multiply(0.60);
    let fg = if selected { selected_fg } else { unselected_fg };

    let resp = ui
        .add(
            Button::new(RichText::new(text).color(fg))
                .fill(Color32::TRANSPARENT)
                .stroke(egui::Stroke::NONE)
                .corner_radius(CORNER_RADIUS),
        )
        .on_hover_cursor(CursorIcon::PointingHand);

    if resp.hovered() {
        ui.painter().rect_filled(resp.rect, CORNER_RADIUS, hover_bg);
        ui.painter().text(
            resp.rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            egui::TextStyle::Button.resolve(ui.style()),
            fg,
        );
    }

    resp
}
