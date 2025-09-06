use crate::{CogsApp, comps::AppComponent, constants::EXPLORE_LINK_TEMPLATE};
use cogs_shared::domain::model::meta::LinkTemplate;
use egui::{Align, Color32, ComboBox, CursorIcon, Direction, Grid, Layout, RichText, Window};
use std::sync::{Arc, Mutex};

pub struct LinkTemplateForm {}

impl AppComponent for LinkTemplateForm {
    type Context = CogsApp;

    /// It shows the form for creating or editing an attribute template.
    /// As params, it expects an `Arc<Mutex<ManagedAttrTemplate>>` in `ui.ctx()`'s data key id named `EXPLORE_LINK_TEMPLATE`.
    fn show(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {
        //
        let ectx = ui.ctx();
        let binding = ectx
            .data(|d| d.get_temp::<Arc<Mutex<LinkTemplate>>>(egui::Id::from(EXPLORE_LINK_TEMPLATE)))
            .clone()
            .unwrap_or_else(|| {
                log::error!("[LinkTemplateForm] Expected id in ui.ctx().data() key id named EXPLORE_LINK_TEMPLATE not found!");
                Arc::new(Mutex::new(LinkTemplate::default()))
            });
        let mut element = binding.lock().unwrap();

        let id = element.id.clone();
        let title: &str;
        match element.id.is_zero() {
            true => {
                title = "New Link Template";
            }
            false => {
                title = "Edit Link Template";
            }
        }

        Window::new(format!("link_templ_form_id_{}", element.id))
            .title_bar(false)
            .resizable(false)
            .min_width(300.0)
            .max_width(400.0)
            .min_height(200.0)
            .max_height(400.0)
            .show(ectx, |ui| {
                ui.vertical(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new(title).size(13.0));
                        if !id.is_zero() {
                            ui.label(RichText::new(format!("   (id: {})", id)).color(Color32::GRAY).size(10.0));
                        }
                    });
                    ui.add_space(20.0);
                    ui.horizontal(|ui| {
                        ui.add_space(14.0);
                        Grid::new(format!("link_templ_form_id_{}_grid", id))
                            .spacing([10.0, 10.0])
                            .num_columns(2)
                            .show(ui, |ui| {
                                ui.label("            Name");
                                // ui.text_edit_singleline(&mut ctx.state.data.curr_attr_template.name);
                                ui.text_edit_singleline(&mut element.name);
                                ui.end_row();
                                ui.label("   Description");
                                ui.text_edit_singleline(&mut element.description);
                                ui.end_row();
                                ui.label("   Target Item");
                                ComboBox::from_id_salt(format!("link_templ_form_id_{}_target", id))
                                    .width(287.0)
                                    .selected_text(element.target_item_template_id.to_string())
                                    .show_ui(ui, |ui| {
                                        // TODO: get all item templates and show them here as `ui.selectable_value`s.
                                        // ui.selectable_value(
                                        // &mut element.target_item_template_id,
                                        // AttributeValueType::Text,
                                        // AttributeValueType::Text.to_string(),
                                        // );
                                    });
                                ui.end_row();
                                ui.label("    Mandatory");
                                ui.checkbox(&mut element.is_required, "");
                                ui.end_row();
                            });
                        ui.add_space(8.0);
                    });

                    ui.add_space(20.0);

                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        ui.add_space(18.0);
                        if ui.button("    Save    ").clicked() {
                            ctx.state
                                .data
                                .save_link_template(element.clone(), ui.ctx(), ctx.sendr.clone());
                            ctx.state.explore.open_windows_attr_template.remove(&id);
                        }
                        ui.add_space(8.0);
                        if ui.button("  Cancel  ").clicked() {
                            ctx.state.explore.open_windows_link_template.remove(&id);
                        }
                        if !element.id.is_zero() {
                            ui.with_layout(
                                Layout::from_main_dir_and_cross_align(Direction::LeftToRight, Align::Min),
                                |ui| {
                                    ui.add_space(18.0);
                                    if ui.button("  Delete   ").clicked() {
                                        ctx.state.data.delete_attr_template(id.clone(), ectx, ctx.sendr.clone());
                                        ctx.state.explore.open_windows_attr_template.remove(&id);
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
