use egui::{ComboBox, Layout, RichText};
use egui_extras::{Size, StripBuilder};
use serde::{Deserialize, Serialize};

use crate::{CogsApp, views::AppView};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum ExploreCategory {
    #[default]
    All,
    Items,
    Templates,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum ExploreKind {
    #[default]
    All,
    Attribute,
    Item,
}

pub struct Explore {}

impl AppView for Explore {
    type Context = CogsApp;

    fn show(_ctx: &mut Self::Context, ectx: &egui::Context) {
        egui::CentralPanel::default().show(ectx, |ui| {
            // The central panel is the region left after adding TopPanel's and SidePanel's

            ui.add_space(10.0);
            ui.heading("Explore");
            ui.add_space(10.0);

            StripBuilder::new(ui)
                .size(Size::remainder().at_least(200.0)) // top/left cell
                .size(Size::remainder().at_least(300.0)) // bottom/right cell
                .horizontal(|mut strip| {
                    // The top/left cell.
                    strip.cell(|ui| {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.label("Category:");
                                let sel_categ = match _ctx.state.explore.category {
                                    ExploreCategory::All => RichText::new("all").italics(),
                                    ExploreCategory::Items => RichText::new("Items"),
                                    ExploreCategory::Templates => RichText::new("Templates"),
                                };
                                ComboBox::from_id_salt("xplore_categ")
                                    .selected_text(sel_categ)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut _ctx.state.explore.kind,
                                            ExploreKind::All,
                                            RichText::new("all").italics(),
                                        );
                                        ui.selectable_value(
                                            &mut _ctx.state.explore.category,
                                            ExploreCategory::Items,
                                            "Items",
                                        );
                                        ui.selectable_value(
                                            &mut _ctx.state.explore.category,
                                            ExploreCategory::Templates,
                                            "Templates",
                                        );
                                    });
                                ui.add_space(10.0);

                                ui.label("Kind:");
                                let sel_kind = match _ctx.state.explore.kind {
                                    ExploreKind::All => RichText::new("all").italics(),
                                    ExploreKind::Item => RichText::new("Item"),
                                    ExploreKind::Attribute => RichText::new("Attribute"),
                                };
                                ComboBox::from_id_salt("xplore_kind")
                                    .selected_text(sel_kind)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut _ctx.state.explore.kind,
                                            ExploreKind::All,
                                            RichText::new("all").italics(),
                                        );
                                        ui.selectable_value(
                                            &mut _ctx.state.explore.kind,
                                            ExploreKind::Item,
                                            "Item",
                                        );
                                        ui.selectable_value(
                                            &mut _ctx.state.explore.kind,
                                            ExploreKind::Attribute,
                                            "Attribute",
                                        );
                                    });

                                ui.add_space(10.0);
                                if ui.button(" + ").clicked() {
                                    //
                                }
                            })
                        });
                    });
                    // The bottom/right cell. It contains a nested strip.
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                            strip.cell(|ui| {
                                ui.vertical(|ui| {
                                    ui.label("Attributes");
                                });
                            });

                            strip.cell(|ui| {
                                ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                                    ui.label("Links");
                                });
                            });
                        });
                    });
                });
        });
    }
}
