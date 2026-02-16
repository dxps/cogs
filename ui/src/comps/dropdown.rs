use egui::{CursorIcon, Id, Popup, Response, RichText, Sense, Style, TextStyle, TextWrapMode, Ui, WidgetText, pos2};

use crate::{
    comps::{menu_row, paint_combo_chevron},
    constants::CORNER_RADIUS,
};

#[derive(Clone, Copy, Debug)]
pub struct DropdownStyle {
    pub width: f32,
    pub height: f32,
    pub gap: f32,
    pub row_width: Option<f32>,
}

impl Default for DropdownStyle {
    fn default() -> Self {
        Self {
            width: 120.0,
            height: 20.0,
            gap: 4.0,
            row_width: Some(120.0),
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
        id: Id,
        selected: &T,
        items: &[DropdownItem<T>],
        style: DropdownStyle,
    ) -> Option<T> {
        let (selected_label, selected_italic) = items
            .iter()
            .find(|it| &it.value == selected)
            .map(|it| (it.label.as_str(), it.italic))
            .unwrap_or(("", false));

        let trigger = Self::trigger(ui, selected_label, style, selected_italic);

        let popup_style = Self::popup_style(ui);
        let mut picked: Option<T> = None;

        Popup::menu(&trigger).id(id).style(popup_style).gap(style.gap).show(|ui| {
            for item in items {
                if menu_row(ui, &item.label, item.italic, style.row_width).clicked() {
                    picked = Some(item.value.clone());
                    ui.close();
                }
            }
        });

        picked
    }

    fn trigger(ui: &mut Ui, label: &str, style: DropdownStyle, italic: bool) -> Response {
        let (rect, resp) = ui.allocate_exact_size(egui::vec2(style.width, style.height), Sense::click());

        let v = ui.visuals();
        let bg = if resp.is_pointer_button_down_on() {
            v.widgets.active.bg_fill
        } else if resp.hovered() {
            v.widgets.hovered.bg_fill
        } else {
            v.widgets.inactive.bg_fill
        };

        ui.painter().rect_filled(rect, CORNER_RADIUS, bg);

        let mut rt = RichText::new(label).color(v.widgets.inactive.fg_stroke.color);
        if italic {
            rt = rt.italics();
        }

        let galley = WidgetText::from(rt).into_galley(ui, Some(TextWrapMode::Extend), f32::INFINITY, TextStyle::Button);
        let text_pos = pos2(rect.left() + 10.0, rect.center().y - galley.size().y * 0.5);
        ui.painter().galley(text_pos, galley, v.widgets.inactive.fg_stroke.color);

        paint_combo_chevron(ui, rect);

        resp.on_hover_cursor(CursorIcon::PointingHand)
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
