use egui::{ComboBox, Layout};
use egui_extras::{Size, StripBuilder};
use serde::{Deserialize, Serialize};

use crate::{CogsApp, views::AppView};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum ExploreCategory {
    #[default]
    Items,
    Templates,
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
                                ui.label("Items type: ");
                                let sel = match _ctx.state.explore_category {
                                    ExploreCategory::Items => "Items",
                                    ExploreCategory::Templates => "Templates",
                                };
                                ComboBox::from_label("").selected_text(sel).show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut _ctx.state.explore_category,
                                        ExploreCategory::Items,
                                        "Items",
                                    );
                                    ui.selectable_value(
                                        &mut _ctx.state.explore_category,
                                        ExploreCategory::Templates,
                                        "Templates",
                                    );
                                });
                            })
                        });
                    });
                    // The bottom/right cell. It contains a nested strip.
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                            strip.cell(|ui| {
                                ui.vertical(|ui| {
                                    ui.label("Item Properties");
                                });
                            });

                            strip.cell(|ui| {
                                ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                                    ui.label("Other Properties");
                                });
                            });
                        });
                    });
                });
        });
    }
}
