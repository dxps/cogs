use egui::{CursorIcon, Sense, WidgetInfo, WidgetType};
use serde::{Deserialize, Serialize};

use crate::constants::CORNER_RADIUS;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttrsLinksTab {
    Attributes,
    Links,
}

pub fn horiz_tab(ui: &mut egui::Ui, text: &str, selected: bool) -> egui::Response {
    let text_color = ui.visuals().text_color();
    let hover_bg = ui.visuals().widgets.hovered.weak_bg_fill;
    let unselected_fg = text_color.gamma_multiply(0.60);
    let fg = if selected { text_color } else { unselected_fg };

    let font_id = egui::TextStyle::Button.resolve(ui.style());
    let galley = ui.painter().layout_no_wrap(text.to_owned(), font_id, fg);
    let desired_size = (galley.size() + 2.0 * ui.spacing().button_padding).max(ui.spacing().interact_size);
    let (rect, resp) = ui.allocate_exact_size(desired_size, Sense::click());

    resp.widget_info(|| WidgetInfo::selected(WidgetType::Button, ui.is_enabled(), selected, text));

    if resp.hovered() {
        ui.painter().rect_filled(rect, CORNER_RADIUS, hover_bg);
    }

    let text_rect = egui::Align2::CENTER_CENTER.align_size_within_rect(galley.size(), rect);
    ui.painter().galley(text_rect.min, galley, fg);

    resp.on_hover_cursor(CursorIcon::PointingHand)
}
