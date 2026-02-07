use crate::constants::{POPUP_ROW_HEIGHT, POPUP_ROW_WIDTH_MIN};

pub fn menu_row(ui: &mut egui::Ui, label: &str, label_italic: bool, width: Option<f32>) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(width.unwrap_or(POPUP_ROW_WIDTH_MIN), POPUP_ROW_HEIGHT),
        egui::Sense::click(),
    );

    let visuals = ui.visuals();
    let row_hovered = ui.rect_contains_pointer(rect);
    let row_down = response.is_pointer_button_down_on();

    let bg = if row_down {
        visuals.widgets.active.bg_fill
    } else if row_hovered {
        visuals.widgets.hovered.bg_fill
    } else {
        egui::Color32::TRANSPARENT
    };

    if bg != egui::Color32::TRANSPARENT {
        ui.painter().rect_filled(rect, 4.0, bg);
    }

    let text_color = if row_hovered {
        visuals.widgets.hovered.fg_stroke.color
    } else {
        visuals.widgets.inactive.fg_stroke.color
    };

    // Paint-only text (no inner widget => no click stealing)
    let mut rt = egui::RichText::new(label).color(text_color);
    if label_italic {
        rt = rt.italics();
    }

    let widget_text: egui::WidgetText = rt.into();
    let galley = widget_text.into_galley(
        ui,
        Some(egui::TextWrapMode::Extend), // no wrapping
        f32::INFINITY,
        egui::TextStyle::Button,
    );

    let text_pos = egui::pos2(rect.left() + 8.0, rect.center().y - galley.size().y * 0.5);
    ui.painter().galley(text_pos, galley, text_color);

    if row_hovered {
        ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
    }

    response
}
