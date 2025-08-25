use std::sync::{Arc, Mutex};

use cogs_shared::domain::model::meta::ItemTemplate;
use egui::{Align, Color32, Direction, Layout, RichText, Window};

use crate::{CogsApp, comps::AppComponent, constants::EXPLORE_ELEMENT};

pub struct ItemTemplateForm {}

impl AppComponent for ItemTemplateForm {
    type Context = CogsApp;

    /// It shows the form for creating or editing an item template.
    /// As params, it expects an `Arc<Mutex<ItemTemplate>>` in `ui.ctx()`'s data key id named `EXPLORE_ELEMENT`.
    fn show(ctx: &mut Self::Context, ui: &mut egui::Ui) {
        //
        let ectx = ui.ctx();
        let binding = ectx
            .data(|d| d.get_temp::<Arc<Mutex<ItemTemplate>>>(egui::Id::from(EXPLORE_ELEMENT)))
            .clone()
            .unwrap_or_default();
        let mut element = binding.lock().unwrap();

        let id = element.id.clone();
        let title: &str;
        match element.id.is_zero() {
            true => {
                title = "New Item Template";
            }
            false => {
                title = "Edit Item Template";
            }
        }

        Window::new(format!("ItemTemplateForm_id_{}", element.id))
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
                            ui.label(
                                RichText::new(format!("   (id: {})", id))
                                    .color(Color32::GRAY)
                                    .size(10.0),
                            );
                        }
                    });
                    ui.add_space(20.0);
                });

                ui.add_space(20.0);

                ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    ui.add_space(18.0);
                    if ui.button("    Save    ").clicked() {
                        // ctx.state
                        // .data
                        // .save_attr_template(element.clone(), ui.ctx(), ctx.sendr.clone());
                        ctx.state.explore.open_windows_item_template.remove(&id);
                    }
                    ui.add_space(8.0);
                    if ui.button("  Cancel  ").clicked() {
                        ctx.state.explore.open_windows_item_template.remove(&id);
                    }
                    if !element.id.is_zero() {
                        ui.with_layout(
                            Layout::from_main_dir_and_cross_align(Direction::LeftToRight, Align::Min),
                            |ui| {
                                ui.add_space(18.0);
                                if ui.button("  Delete   ").clicked() {
                                    // ctx.state.data.delete_attr_template(id.clone(), ectx, ctx.sendr.clone());
                                    ctx.state.explore.open_windows_item_template.remove(&id);
                                }
                            },
                        );
                    }
                });
                ui.add_space(12.0);
            });
    }
}
