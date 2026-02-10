use crate::{CogsApp, comps::AppComponent, constants::EXPLORE_ELEMENT};
use cogs_shared::domain::model::{
    Action, Id,
    meta::{AttrTemplate, AttributeValueType},
};
use egui::{Align, Button, Color32, ComboBox, CursorIcon, Direction, Grid, Label, Layout, RichText, Window, vec2};
use std::sync::{Arc, Mutex};

pub struct AttrTemplateForm {}

impl AppComponent for AttrTemplateForm {
    type Context = CogsApp;

    /// It shows the form for creating or editing an attribute template.\
    /// In its `ui.ctx()`'s `data` it expects an `Arc<Mutex<AttrTemplate>>` for the id named `EXPLORE_ELEMENT`.
    fn show(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {
        //
        let ectx = ui.ctx();
        let binding = ectx
            .data(|d| d.get_temp::<Arc<Mutex<AttrTemplate>>>(egui::Id::from(EXPLORE_ELEMENT)))
            .clone()
            .unwrap_or_default();
        let mut element = binding.lock().unwrap();
        let id = element.id.clone();
        let act_id = egui::Id::from(format!("attr_tmpl_id_{}_action", id));

        let action = match id.is_zero() {
            true => Action::Create,
            false => ectx.data(|d| d.get_temp::<Action>(act_id)).unwrap_or(Action::View),
        };

        let title = match action {
            Action::Create => "New Attribute Template",
            Action::Edit => "Edit Attribute Template",
            _ => "View Attribute Template",
        };

        let focus_id = egui::Id::new("new_attr_template_form_focus_name_once");
        let focus_name_once = ui.ctx().data_mut(|d| d.get_temp::<bool>(focus_id).unwrap_or(true));

        Window::new(format!("attr_tmpl_form_{}_win", element.id))
            .title_bar(false)
            .resizable(false)
            .fixed_size(vec2(400.0, 300.0))
            .show(ectx, |ui| {
                ui.vertical(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_enabled(false, Label::new(RichText::new(title).size(13.0)));
                        if !id.is_zero() {
                            ui.add_enabled(
                                action.is_edit(),
                                Label::new(RichText::new(format!("   (id: {})", id)).color(Color32::GRAY).size(10.0)),
                            );
                        }
                    });
                    ui.add_space(20.0);
                    ui.horizontal(|ui| {
                        ui.add_space(14.0);
                        Grid::new(format!("attr_tmpl_form_{}_grid", id))
                            .spacing([10.0, 10.0])
                            .num_columns(2)
                            .show(ui, |ui| {
                                ui.add_enabled(false, Label::new("            Name"));
                                let resp = ui.add(egui::TextEdit::singleline(&mut element.name).interactive(!action.is_view()));
                                if action.is_create() && focus_name_once {
                                    resp.request_focus();
                                    ui.ctx().data_mut(|d| d.insert_temp(focus_id, false));
                                }
                                ui.end_row();

                                ui.add_enabled(false, Label::new("   Description"));
                                ui.add(egui::TextEdit::singleline(&mut element.description).interactive(!action.is_view()));
                                ui.end_row();

                                ui.add_enabled(false, Label::new("    Value Type"));
                                if action.is_view() {
                                    ui.add(egui::TextEdit::singleline(&mut element.value_type.to_string()).interactive(false));
                                } else {
                                    ComboBox::from_id_salt(format!("atf_val_type_{}", id))
                                        .width(287.0)
                                        .selected_text(element.value_type.to_string())
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                &mut element.value_type,
                                                AttributeValueType::Text,
                                                AttributeValueType::Text.to_string(),
                                            );
                                            ui.selectable_value(
                                                &mut element.value_type,
                                                AttributeValueType::SmallInteger,
                                                AttributeValueType::SmallInteger.to_string(),
                                            );
                                        });
                                }
                                ui.end_row();
                                ui.add_enabled(false, Label::new("Default value"));
                                ui.add(egui::TextEdit::singleline(&mut element.default_value).interactive(!action.is_view()));
                                ui.end_row();
                                ui.add_enabled(false, Label::new("    Mandatory"));
                                if action.is_view() {
                                    ui.add_enabled(false, egui::Checkbox::new(&mut element.is_required, ""));
                                } else {
                                    ui.checkbox(&mut element.is_required, "");
                                }
                                ui.end_row();
                            });
                        ui.add_space(8.0);
                    });

                    ui.add_space(20.0);

                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        ui.add_space(18.0);
                        if action.is_view() {
                            if ui.button("    Edit    ").clicked() {
                                ectx.data_mut(|d| d.insert_temp(act_id, Action::Edit));
                            }
                        } else {
                            let enabled = !element.name.is_empty();
                            let resp = ui
                                .add_enabled(enabled, Button::new("    Save    "))
                                .on_disabled_hover_text("Provide the at least a name before saving.");
                            if resp.clicked() {
                                ctx.state
                                    .data
                                    .save_attr_template(element.clone(), ui.ctx(), ctx.sendr.clone());
                                cleanup(ctx, ectx, &id, act_id, focus_id);
                            }
                        }
                        ui.add_space(8.0);
                        if ui.button("  Cancel  ").clicked() {
                            cleanup(ctx, ectx, &id, act_id, focus_id);
                        }
                        if !element.id.is_zero() {
                            ui.with_layout(
                                Layout::from_main_dir_and_cross_align(Direction::LeftToRight, Align::Min),
                                |ui| {
                                    ui.add_space(18.0);
                                    if ui.button("  Delete   ").clicked() {
                                        ctx.state.data.delete_attr_template(id.clone(), ectx, ctx.sendr.clone());
                                        cleanup(ctx, ectx, &id, act_id, focus_id);
                                    }
                                },
                            );
                        }
                    });
                    ui.add_space(12.0);
                })
                .response
                .on_hover_cursor(CursorIcon::Grab);
            });
    }
}

fn cleanup(ctx: &mut CogsApp, ectx: &egui::Context, id: &Id, act_id: egui::Id, focus_id: egui::Id) {
    ctx.state.explore.open_windows_attr_template.remove(id);
    ectx.data_mut(|d| d.remove::<Action>(act_id));
    ectx.data_mut(|d| d.remove::<bool>(focus_id));
}
