use crate::{
    CogsApp,
    comps::AppComponent,
    constants::{CORNER_RADIUS, EXPLORE_ELEMENT},
};
use cogs_shared::domain::model::{
    Action, Id,
    meta::{AttrTemplate, ItemTemplate},
};
use egui::{Align, Button, Color32, ComboBox, CursorIcon, Direction, Frame, Grid, Label, Layout, RichText, TextEdit, Window};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct ItemTemplateForm;

struct FormUiState {
    id: Id,
    act_id: egui::Id,
    focus_id: egui::Id,
    action: Action,
    title: &'static str,
    focus_name_once: bool,
}

impl FormUiState {
    fn from_ctx(ectx: &egui::Context, element: &ItemTemplate) -> Self {
        let id = element.id.clone();
        let act_id = egui::Id::from(format!("item_tmpl_form_{}_action", id));
        let action = if id.is_zero() {
            Action::Create
        } else {
            ectx.data(|d| d.get_temp::<Action>(act_id)).unwrap_or(Action::View)
        };

        let title = match action {
            Action::Create => "New Item Template",
            Action::Edit => "Edit Item Template",
            _ => "View Item Template",
        };

        let focus_id = egui::Id::new("new_item_template_form_focus_name_once");
        let focus_name_once = ectx.data_mut(|d| d.get_temp::<bool>(focus_id).unwrap_or(true));

        Self {
            id,
            act_id,
            focus_id,
            action,
            title,
            focus_name_once,
        }
    }
}

impl ItemTemplateForm {
    fn reorder_attrs(element: &mut ItemTemplate, from_idx: usize, to_idx: usize) {
        let attr = element.attributes.remove(from_idx);
        element.attributes.insert(to_idx, attr);
    }

    fn render_header(ui: &mut egui::Ui, s: &FormUiState) {
        ui.vertical_centered(|ui| {
            ui.add_enabled(false, Label::new(RichText::new(s.title).size(13.0)));
            if !s.id.is_zero() {
                ui.add_enabled(
                    s.action.is_edit(),
                    Label::new(RichText::new(format!("(id: {})", s.id)).color(Color32::GRAY).size(10.0)),
                );
            }
        });
    }

    fn render_form_grid(
        app: &mut CogsApp,
        ui: &mut egui::Ui,
        ectx: &egui::Context,
        element: &mut ItemTemplate,
        s: &mut FormUiState,
    ) {
        ui.horizontal(|ui| {
            ui.add_space(14.0);

            Grid::new(format!("item_tmpl_id_{}_grid", s.id))
                .spacing([10.0, 10.0])
                .num_columns(2)
                .show(ui, |ui| {
                    Self::row_name(ui, ectx, element, s);
                    Self::row_description(ui, element, s);
                    Self::row_listing_attr(ui, element, s);
                    Self::row_attributes(ui, element, s);
                    ui.label("");
                    ui.end_row();

                    if s.action != Action::View {
                        Self::row_add_attr_template(app, ui, element, s);
                        ui.end_row();
                    }
                });

            ui.add_space(8.0);
        });
    }

    fn render_footer_buttons(
        app: &mut CogsApp,
        ui: &mut egui::Ui,
        ectx: &egui::Context,
        element: &mut ItemTemplate,
        s: &FormUiState,
    ) {
        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
            ui.add_space(18.0);

            if s.action.is_view() {
                if ui.button("    Edit    ")
                    .on_hover_cursor(CursorIcon::PointingHand)
                    .clicked()
                {
                    ectx.data_mut(|d| d.insert_temp(s.act_id, Action::Edit));
                }
            } else {
                let enabled = !element.name.is_empty()
                    && !element.attributes.is_empty()
                    && !element.listing_attr.is_default();

                let resp = ui
                    .add_enabled(enabled, Button::new("    Save    "))
                    .on_hover_cursor(CursorIcon::PointingHand)
                    .on_disabled_hover_text(
                        "Provide the following parts before saving:\n- name\n- listing attribute template\n- one or more attribute templates",
                    );

                if resp.clicked() {
                    app.state
                        .data
                        .save_item_template(element.clone(), ui.ctx(), app.sendr.clone());
                    shutdown(app, ectx, &s.id, s.act_id, s.focus_id);
                }
            }

            ui.add_space(8.0);

            if ui.button("  Cancel  ")
                .on_hover_cursor(CursorIcon::PointingHand)
                .clicked()
            {
                shutdown(app, ectx, &s.id, s.act_id, s.focus_id);
            }

            if !element.id.is_zero() {
                ui.with_layout(
                    Layout::from_main_dir_and_cross_align(Direction::LeftToRight, Align::Min),
                    |ui| {
                        ui.add_space(18.0);
                        if ui.button("  Delete  ")
                            .on_hover_cursor(CursorIcon::PointingHand)
                            .clicked()
                        {
                            app.state
                                .data
                                .delete_item_template(s.id.clone(), ectx, app.sendr.clone());
                            shutdown(app, ectx, &s.id, s.act_id, s.focus_id);
                        }
                    },
                );
            }
        });
    }

    fn row_name(ui: &mut egui::Ui, ectx: &egui::Context, element: &mut ItemTemplate, s: &mut FormUiState) {
        ui.add_enabled(false, Label::new("                                   Name"));
        let resp = ui.add_sized(
            [250.0, ui.spacing().interact_size.y],
            TextEdit::singleline(&mut element.name).interactive(!s.action.is_view()),
        );
        if s.action.is_create() && s.focus_name_once {
            resp.request_focus();
            ectx.data_mut(|d| d.insert_temp(s.focus_id, false));
            s.focus_name_once = false;
        }
        ui.end_row();
    }

    fn row_description(ui: &mut egui::Ui, element: &mut ItemTemplate, s: &FormUiState) {
        ui.add_enabled(false, Label::new("                         Description"));
        ui.add(TextEdit::singleline(&mut element.description).interactive(!s.action.is_view()));
        ui.end_row();
    }

    fn row_listing_attr(ui: &mut egui::Ui, element: &mut ItemTemplate, s: &FormUiState) {
        ui.add_enabled(false, Label::new("Listing Attribute Template"));
        if s.action.is_view() {
            ui.add(TextEdit::singleline(&mut element.listing_attr.name).interactive(false));
        } else {
            ComboBox::from_id_salt(format!("item_templ_form_{}_listing_attr_", s.id))
                .width(250.0)
                .selected_text(element.listing_attr.name.clone())
                .show_ui(ui, |ui| {
                    for attr in &element.attributes.clone() {
                        ui.selectable_value(&mut element.listing_attr, attr.clone(), attr.name.clone());
                    }
                });
        }
        ui.end_row();
    }

    fn row_attributes(ui: &mut egui::Ui, element: &mut ItemTemplate, s: &FormUiState) {
        ui.add_enabled(false, Label::new("                           Attributes"));

        if element.attributes.is_empty() {
            ui.label(RichText::new("None").italics().color(Color32::GRAY));
            ui.end_row();
            return;
        }

        ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([true; 2])
                .vscroll(false)
                .show(ui, |ui| {
                    if s.action.is_view() {
                        let mut attrs_text = element
                            .attributes
                            .iter()
                            .map(|a| a.name.clone())
                            .collect::<Vec<_>>()
                            .join("\n");
                        let rows = element.attributes.len().max(1);
                        ui.add(
                            TextEdit::multiline(&mut attrs_text)
                                .frame(true)
                                .interactive(false)
                                .desired_rows(rows)
                                .desired_width(f32::INFINITY),
                        );
                    } else {
                        Self::render_dnd_attr_list(ui, element);
                    }
                });
        });

        ui.end_row();
    }

    fn render_dnd_attr_list(ui: &mut egui::Ui, element: &mut ItemTemplate) {
        let frame = Frame::default().corner_radius(CORNER_RADIUS).inner_margin(4.0);
        let mut from_idx = None::<usize>;
        let mut to_idx = None::<usize>;

        ui.dnd_drop_zone::<usize, ()>(frame, |ui| {
            ui.set_min_width(242.0);

            for (idx, item) in element.attributes.iter().enumerate() {
                let row_id = egui::Id::new(("item_tmpl_attr_row", element.id.clone(), item.id.clone(), idx));
                let item_idx = idx;

                let response = ui
                    .push_id(row_id, |ui| ui.dnd_drag_source(row_id, item_idx, |ui| ui.label(&item.name)))
                    .response;

                if let (Some(pointer), Some(hovered_idx)) =
                    (ui.input(|i| i.pointer.interact_pos()), response.dnd_hover_payload::<usize>())
                {
                    let rect = response.rect;
                    let stroke = egui::Stroke::new(1.4, Color32::WHITE);

                    let drop_idx = if *hovered_idx == item_idx {
                        item_idx
                    } else if pointer.y < rect.center().y {
                        ui.painter().hline(rect.x_range().shrink(1.0), rect.top(), stroke);
                        item_idx
                    } else {
                        ui.painter().hline(rect.x_range().shrink(1.0), rect.bottom(), stroke);
                        item_idx
                    };

                    let attrs_len = element.attributes.len();
                    if let Some(drag_idx) = response.dnd_release_payload::<usize>() {
                        from_idx = Some(*drag_idx);
                        to_idx = Some(if drop_idx == attrs_len { attrs_len - 1 } else { drop_idx });
                    }
                }
            }
        });

        if let (Some(from), Some(to)) = (from_idx, to_idx) {
            if from != to {
                Self::reorder_attrs(element, from, to);
            }
        }
    }

    fn row_add_attr_template(app: &mut CogsApp, ui: &mut egui::Ui, element: &mut ItemTemplate, s: &FormUiState) {
        ui.add_enabled(false, Label::new("     Add Attribute Template"));

        ui.horizontal(|ui| {
            let curr_attr_tmpl = app.state.explore.item_template_cu_add_attr_template.clone();

            let response = ComboBox::from_id_salt(format!("item_templ_form_{}_add_attr_", s.id))
                .width(220.0)
                .selected_text(selected_attr_name(&curr_attr_tmpl, &element.id))
                .show_ui(ui, |ui| {
                    let selected_for_element = app
                        .state
                        .explore
                        .item_template_cu_add_attr_template
                        .entry(element.id.clone())
                        .or_insert(None);

                    for at in app.state.data.get_attr_templates() {
                        if element.attributes.iter().all(|a| a.id != at.id) {
                            ui.selectable_value(selected_for_element, Some(at.clone()), at.name.clone());
                        }
                    }
                })
                .response;

            if let Some(drag_idx) = response.dnd_release_payload::<usize>() {
                element.attributes.remove(*drag_idx);
                if element.attributes.is_empty() {
                    element.listing_attr = Default::default();
                }
            }

            let has_selected = app
                .state
                .explore
                .item_template_cu_add_attr_template
                .get(&element.id)
                .and_then(|o| o.as_ref())
                .is_some();

            let btn = ui
                .add_enabled(has_selected, Button::new(" + "))
                .on_disabled_hover_text("Select an attribute template first");

            if btn.clicked() {
                if let Some(attr) = app
                    .state
                    .explore
                    .item_template_cu_add_attr_template
                    .get(&element.id)
                    .and_then(|o| o.clone())
                {
                    element.attributes.push(attr);
                    app.state
                        .explore
                        .item_template_cu_add_attr_template
                        .insert(element.id.clone(), None);
                }
            }
        });
    }
}

impl AppComponent for ItemTemplateForm {
    type Context = CogsApp;

    fn show(ctx: &mut Self::Context, ui: &mut egui::Ui) {
        let ectx = ui.ctx();

        if !ctx.state.data.has_fetched_attr_templates() {
            ctx.state.data.fetch_all_attr_templates(ectx, ctx.sendr.clone());
        }

        let binding = ectx
            .data(|d| d.get_temp::<Arc<Mutex<ItemTemplate>>>(egui::Id::from(EXPLORE_ELEMENT)))
            .clone()
            .unwrap_or_default();

        let mut element = binding.lock().unwrap();
        let mut s = FormUiState::from_ctx(ectx, &element);

        Window::new(format!("item_tmpl_form_{}_win", element.id))
            .title_bar(false)
            .resizable(false)
            .min_width(300.0)
            .max_width(400.0)
            .min_height(200.0)
            .max_height(400.0)
            .show(ectx, |ui| {
                ui.vertical(|ui| {
                    Self::render_header(ui, &s);
                    ui.add_space(20.0);
                    Self::render_form_grid(ctx, ui, ectx, &mut element, &mut s);
                })
                .response
                .on_hover_cursor(CursorIcon::Grab);

                ui.add_space(20.0);
                Self::render_footer_buttons(ctx, ui, ectx, &mut element, &s);
                ui.add_space(12.0);
            });
    }
}

fn selected_attr_name(map: &HashMap<Id, Option<AttrTemplate>>, id: &Id) -> String {
    map.get(id)
        .and_then(|o| o.as_ref())
        .map(|at| at.name.clone())
        .unwrap_or_default()
}

fn shutdown(ctx: &mut CogsApp, ectx: &egui::Context, id: &Id, act_id: egui::Id, focus_id: egui::Id) {
    ctx.state.explore.open_windows_item_template.remove(id);
    ectx.data_mut(|d| d.remove::<Action>(act_id));
    ectx.data_mut(|d| d.remove::<bool>(focus_id));
}
