use egui::{CursorIcon, Id, Popup, Response, RichText, Sense, Style, TextStyle, TextWrapMode, Ui, WidgetText, pos2};

use crate::{
    comps::{menu_row, paint_combo_chevron},
    constants::CORNER_RADIUS,
};

#[derive(Clone, Copy, Debug)]
pub struct DropdownStyle {
    pub width: Option<f32>, // None => auto
    pub min_width: f32,
    pub max_width: Option<f32>,
    pub height: f32,
    pub gap: f32,
    pub row_width: Option<f32>,
    pub hpad: f32,          // left text padding
    pub chevron_space: f32, // reserved space on right for chevron
}

impl Default for DropdownStyle {
    fn default() -> Self {
        Self {
            width: None, // auto by default
            min_width: 120.0,
            max_width: None, // no cap unless caller sets one
            height: 20.0,
            gap: 4.0,
            row_width: None, // use trigger width by default
            hpad: 10.0,
            chevron_space: 24.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DropdownItem<T: Clone> {
    pub label: String,
    pub value: T,
    pub italic: bool,
}

impl<T: Clone> DropdownItem<T> {
    pub fn new(label: impl Into<String>, value: T) -> Self {
        Self {
            label: label.into(),
            value,
            italic: false,
        }
    }

    pub fn italic(mut self, yes: bool) -> Self {
        self.italic = yes;
        self
    }
}

pub struct Dropdown;

impl Dropdown {
    pub fn show<T: Clone + PartialEq>(
        ui: &mut Ui,
        id: egui::Id,
        selected: &T,
        items: &[DropdownItem<T>],
        style: DropdownStyle,
    ) -> Option<T> {
        let (selected_label, selected_italic) = items
            .iter()
            .find(|it| &it.value == selected)
            .map(|it| (it.label.as_str(), it.italic))
            .unwrap_or(("", false));

        let resolved_width = resolve_width(ui, items, selected_label, style);
        let row_width = style.row_width.unwrap_or(resolved_width);

        let trigger = Self::trigger(ui, selected_label, resolved_width, style, selected_italic);
        let popup_style = Self::popup_style(ui);

        let mut picked: Option<T> = None;
        egui::Popup::menu(&trigger)
            .id(id)
            .style(popup_style)
            .gap(style.gap)
            .show(|ui| {
                for item in items {
                    if crate::comps::menu_row(ui, &item.label, item.italic, Some(row_width)).clicked() {
                        picked = Some(item.value.clone());
                        ui.close();
                    }
                }
            });

        picked
    }

    fn trigger(
        ui: &mut Ui,
        label: &str,
        width: f32,           // <- resolved concrete width
        style: DropdownStyle, // still used for height/padding
        italic: bool,
    ) -> egui::Response {
        let (rect, resp) = ui.allocate_exact_size(egui::vec2(width, style.height), egui::Sense::click());

        let v = ui.visuals();
        let bg = if resp.is_pointer_button_down_on() {
            v.widgets.active.bg_fill
        } else if resp.hovered() {
            v.widgets.hovered.bg_fill
        } else {
            v.widgets.inactive.bg_fill
        };

        ui.painter().rect_filled(rect, crate::constants::CORNER_RADIUS, bg);

        let mut rt = egui::RichText::new(label).color(v.widgets.inactive.fg_stroke.color);
        if italic {
            rt = rt.italics();
        }

        let galley =
            egui::WidgetText::from(rt).into_galley(ui, Some(egui::TextWrapMode::Extend), f32::INFINITY, egui::TextStyle::Button);

        let text_pos = egui::pos2(rect.left() + style.hpad, rect.center().y - galley.size().y * 0.5);
        ui.painter().galley(text_pos, galley, v.widgets.inactive.fg_stroke.color);

        crate::comps::paint_combo_chevron(ui, rect);

        resp.on_hover_cursor(egui::CursorIcon::PointingHand)
    }

    fn popup_style(ui: &Ui) -> Style {
        let mut s = ui.style().as_ref().clone();
        let v = ui.visuals();

        s.visuals.window_fill = v.widgets.inactive.bg_fill;
        s.visuals.panel_fill = v.widgets.inactive.bg_fill;
        s.visuals.extreme_bg_color = v.widgets.inactive.bg_fill;
        s.visuals.widgets.inactive.bg_fill = v.widgets.inactive.bg_fill;
        s.visuals.widgets.hovered.bg_fill = v.widgets.hovered.bg_fill;
        s.visuals.widgets.active.bg_fill = v.widgets.active.bg_fill;
        s.visuals.selection.bg_fill = v.selection.bg_fill;

        s
    }
}

fn resolve_width<T: Clone + PartialEq>(
    ui: &mut Ui,
    items: &[DropdownItem<T>],
    selected_label: &str,
    style: DropdownStyle,
) -> f32 {
    if let Some(w) = style.width {
        return w.max(style.min_width);
    }

    let font_id = egui::TextStyle::Button.resolve(ui.style());

    let mut max_text_w = text_width(ui, &font_id, selected_label);
    for it in items {
        let w = text_width(ui, &font_id, &it.label);
        if w > max_text_w {
            max_text_w = w;
        }
    }

    let mut w = style.hpad + max_text_w + style.chevron_space;
    if w < style.min_width {
        w = style.min_width;
    }
    if let Some(max_w) = style.max_width {
        w = w.min(max_w);
    }
    w
}

fn text_width(ui: &Ui, font_id: &egui::FontId, text: &str) -> f32 {
    ui.painter()
        .layout_no_wrap(text.to_owned(), font_id.clone(), ui.visuals().text_color())
        .size()
        .x
}
