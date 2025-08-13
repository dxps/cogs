use crate::{CogsApp, comps::AppComponent};
use catppuccin_egui::Theme;
use egui::{Color32, RichText, Ui};
use egui_extras::{Column, TableBuilder};

pub struct ExploreTable {}

impl AppComponent for ExploreTable {
    type Context = CogsApp;

    fn show(ctx: &mut Self::Context, ui: &mut Ui) {
        //
        if !ctx.state.data_mgmt.fetch_done {
            match ctx.state.explore.category {
                crate::views::ExploreCategory::Items => {
                    // TODO
                    // ctx.state.data_mgmt.get_all_items(ectx);
                }
                crate::views::ExploreCategory::Templates => {
                    // TODO
                    ctx.state.data_mgmt.get_all_attr_template(ui.ctx(), ctx.sendr.clone());
                }
            }
        }

        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.add_space(20.0);

            let available_height = ui.available_height();
            let table = TableBuilder::new(ui)
                .striped(false)
                .resizable(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::auto().at_least(200.0).clip(true).resizable(true))
                .column(Column::auto())
                .column(Column::remainder())
                .min_scrolled_height(0.0)
                .max_scroll_height(available_height);

            let table = table.header(20.0, |mut header| {
                header.col(|ui| {
                    ui.label(RichText::new("type").color(Color32::GRAY));
                });
                header.col(|ui| {
                    ui.label(RichText::new("name").color(Color32::GRAY));
                });
                header.col(|ui| {
                    ui.label(RichText::new("value type").color(Color32::GRAY));
                });
                header.col(|ui| {
                    ui.label(RichText::new("description").color(Color32::GRAY));
                });
            });

            match ctx.state.explore.category {
                crate::views::ExploreCategory::Items => {
                    // TODO
                }
                crate::views::ExploreCategory::Templates => {
                    table.body(|mut body| {
                        for template in &ctx.state.data_mgmt.fetched_attr_templates {
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(RichText::new("attribute template").color(Color32::GRAY));
                                });
                                row.col(|ui| {
                                    ui.label(format!("{}", template.name));
                                });
                                row.col(|ui| {
                                    ui.label(format!("{}", template.value_type));
                                });
                                row.col(|ui| {
                                    ui.label(format!("{}", template.description));
                                });
                            });
                        }
                    });
                }
            }
        });
    }
}
