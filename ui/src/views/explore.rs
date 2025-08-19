use crate::{
    CogsApp, ManagedAttrTemplate,
    comps::{AppComponent, AttrTemplateForm, ExploreTable},
    views::AppView,
};
use cogs_shared::domain::model::{Id, meta::Kind};
use egui::{ComboBox, CursorIcon, Layout, Popup, RichText, Sense};
use egui_extras::{Size, StripBuilder};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum ExploreCategory {
    #[default]
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

    fn show(ctx: &mut Self::Context, ectx: &egui::Context) {
        //
        // The central panel is the region left after adding TopPanel's and SidePanel's
        egui::CentralPanel::default().show(ectx, |ui| {
            ui.add_space(10.0);
            ui.heading("Explore");
            ui.add_space(20.0);

            StripBuilder::new(ui)
                .size(Size::remainder().at_least(200.0)) // top/left cell
                .size(Size::remainder().at_least(300.0)) // bottom/right cell
                .horizontal(|mut strip| {
                    // The top/left cell.
                    strip.cell(|ui| {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.label("Category:");
                                let sel_categ = match ctx.state.explore.category {
                                    ExploreCategory::Items => RichText::new("Items"),
                                    ExploreCategory::Templates => RichText::new("Templates"),
                                };
                                ComboBox::from_id_salt("xplore_categ")
                                    .selected_text(sel_categ)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut ctx.state.explore.category,
                                            ExploreCategory::Items,
                                            "Items",
                                        );
                                        ui.selectable_value(
                                            &mut ctx.state.explore.category,
                                            ExploreCategory::Templates,
                                            "Templates",
                                        );
                                    });
                                ui.add_space(10.0);

                                ui.label("Kind:");
                                let sel_kind = match ctx.state.explore.kind {
                                    ExploreKind::All => RichText::new("all").italics(),
                                    ExploreKind::Item => RichText::new("Item"),
                                    ExploreKind::Attribute => RichText::new("Attribute"),
                                };
                                ComboBox::from_id_salt("xplore_kind")
                                    .selected_text(sel_kind)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut ctx.state.explore.kind,
                                            ExploreKind::All,
                                            RichText::new("all").italics(),
                                        );
                                        if ctx.state.explore.category == ExploreCategory::Templates {
                                            ui.selectable_value(&mut ctx.state.explore.kind, ExploreKind::Item, "Item");
                                            ui.selectable_value(
                                                &mut ctx.state.explore.kind,
                                                ExploreKind::Attribute,
                                                "Attribute",
                                            );
                                        }
                                    });

                                ui.add_space(10.0);

                                let btn = ui.button(" + ").interact(Sense::click());
                                ui.horizontal_top(|_ui| {
                                    Popup::menu(&btn)
                                        .id(egui::Id::new("xplore_add_popup"))
                                        .gap(5.0)
                                        .show(|ui| {
                                            if ui.label(" Item ").on_hover_cursor(CursorIcon::PointingHand).clicked() {
                                                ctx.state
                                                    .explore
                                                    .open_windows
                                                    .insert((Kind::Item, Id::default()), "".into());
                                            };
                                            ui.separator();
                                            ui.menu_button("Template", |ui| {
                                                if ui
                                                    .label("Item Template")
                                                    .on_hover_cursor(CursorIcon::PointingHand)
                                                    .clicked()
                                                {
                                                    ctx.state
                                                        .explore
                                                        .open_windows
                                                        .insert((Kind::ItemTemplate, Id::default()), "".into());
                                                };
                                                if ui
                                                    .label("Attribute Template")
                                                    .on_hover_cursor(CursorIcon::PointingHand)
                                                    .clicked()
                                                {
                                                    ctx.state
                                                        .explore
                                                        .open_windows
                                                        .insert((Kind::AttributeTemplate, Id::default()), "".into());
                                                };
                                            });
                                        });
                                });
                            })
                        });

                        ExploreTable::show(ctx, ui);

                        for ((kind, _id), elem_str) in ctx.state.explore.open_windows.clone().iter() {
                            match kind {
                                // TODO
                                // Kind::ItemTemplate => ItemTemplateForm::show(ctx, ui, id),
                                Kind::AttributeTemplate => {
                                    let element = ManagedAttrTemplate::from(elem_str);
                                    log::debug!("[explore.show] opening AttrTemplateForm for element: {:?}", element);
                                    ectx.data_mut(|d| d.insert_temp(egui::Id::from("attr_template"), element));
                                    AttrTemplateForm::show(ctx, ui);
                                }
                                _ => {}
                            }
                        }
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
