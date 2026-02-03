use crate::{
    CogsApp,
    comps::{
        AppComponent, AttrTemplateForm, AttrTemplateProps, ExploreTable, ItemTemplateForm, ItemTemplateProps, LinkTemplateForm,
    },
    constants::{EXPLORE_ELEMENT, ICON_HELP},
    views::AppView,
};
use cogs_shared::domain::model::{
    Id,
    meta::{AttrTemplate, ItemTemplate, Kind, LinkTemplate},
};
use egui::{Color32, ComboBox, CursorIcon, Popup, RichText, Sense};
use egui_extras::{Size, StripBuilder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

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
    Link,
    Item,
}

pub struct Explore {}

impl AppView for Explore {
    type Context = CogsApp;

    fn show(ctx: &mut Self::Context, ectx: &egui::Context) {
        //
        // The central panel is the region left after adding TopPanel's and SidePanel's.
        let style = ectx.style();
        egui::CentralPanel::default()
        .frame(egui::Frame::central_panel(&style).inner_margin(egui::Margin::symmetric(20, 0)))
        .show(ectx, |ui| {
            ui.add_space(17.0);
            ui.label(
                RichText::new(
                    "In this view, you can explore the elements that exist in the system. Click an element to view its properties on the right, double click it to edit or delete.",
                )
                .color(Color32::GRAY),
            );
            ui.add_space(20.0);

            StripBuilder::new(ui)
                .size(Size::relative(0.6).at_least(500.0)) // left
                .size(Size::exact(20.0)) // middle
                .size(Size::remainder().at_least(80.0)) // /right
                .horizontal(|mut strip| {
                    // The left cell.
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
                                        ui.selectable_value(&mut ctx.state.explore.category, ExploreCategory::Items, "Items");
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
                                    ExploreKind::Link => RichText::new("Link"),
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
                                            ui.selectable_value(&mut ctx.state.explore.kind, ExploreKind::Attribute, "Attribute");
                                            ui.selectable_value(&mut ctx.state.explore.kind, ExploreKind::Link, "Link");
                                        }
                                    });

                                ui.label(RichText::new(ICON_HELP).color(Color32::GRAY).size(10.0))
                                    .on_hover_text("If category is 'Items', you may filter by their templates.\nIf category is 'Templates', you may filter by their types.")
                                    .on_hover_cursor(CursorIcon::Help);
                                ui.add_space(20.0);

                                let btn = ui.button(" + ").interact(Sense::click())
                                    .on_hover_text_at_pointer("Add")
                                    .on_hover_cursor(CursorIcon::PointingHand);

                                ui.horizontal_top(|_ui| {
                                    Popup::menu(&btn).id(egui::Id::new("xplore_add_popup")).gap(5.0).show(|ui| {
                                        if ui.label(" Item ").on_hover_cursor(CursorIcon::PointingHand).clicked() {
                                            // TODO: open the item form for creating one.
                                        };
                                        ui.separator();
                                        ui.menu_button("Template", |ui| {
                                            if ui
                                                .label("Item Template").on_hover_cursor(CursorIcon::PointingHand).clicked() {
                                                    ctx.state
                                                    .explore
                                                    .open_windows_item_template
                                                    .insert(Id::default(), Arc::new(Mutex::new(ItemTemplate::default())));
                                            };
                                            if ui
                                                .label("Attribute Template")
                                                .on_hover_cursor(CursorIcon::PointingHand)
                                                .clicked()
                                            {
                                                ctx.state
                                                    .explore
                                                    .open_windows_attr_template
                                                    .insert(Id::default(), Arc::new(Mutex::new(AttrTemplate::default())));
                                            };
                                            if ui
                                                .label("Link Template")
                                                .on_hover_cursor(CursorIcon::PointingHand)
                                                .clicked()
                                            {
                                                ctx.state
                                                    .explore
                                                    .open_windows_link_template
                                                    .insert(Id::default(), Arc::new(Mutex::new(LinkTemplate::default())));
                                            };
                                        });
                                    });
                                });
                            })
                        });

                        ExploreTable::show(ctx, ui);

                        for (_, element) in ctx.state.explore.open_windows_item_template.clone().iter() {
                            ectx.data_mut(|d| d.insert_temp(egui::Id::from(EXPLORE_ELEMENT), element.clone()));
                            ItemTemplateForm::show(ctx, ui);
                        }
                        for (_, element) in ctx.state.explore.open_windows_attr_template.clone().iter() {
                            ectx.data_mut(|d| d.insert_temp(egui::Id::from(EXPLORE_ELEMENT), element.clone()));
                            AttrTemplateForm::show(ctx, ui);
                        }
                        for (_, element) in ctx.state.explore.open_windows_link_template.clone().iter() {
                            ectx.data_mut(|d| d.insert_temp(egui::Id::from(EXPLORE_ELEMENT), element.clone()));
                            LinkTemplateForm::show(ctx, ui);
                        }
                    });

                    strip.cell(|_| {}); // Just a space in the middle.

                    // The right cell.
                    strip.cell(|ui| {
                        ui.vertical(|ui| {
                            ui.add_space(45.0);
                            if let Some((kind, id)) = &ctx.state.explore.curr_sel_elem {
                                match kind {
                                    Kind::AttributeTemplate => {
                                        for elem in ctx.state.data.get_attr_templates().iter() {
                                            if elem.id == *id {
                                                ectx.data_mut(|d| {
                                                    d.insert_temp(egui::Id::from(EXPLORE_ELEMENT), elem.clone())
                                                });
                                                break;
                                            }
                                        }
                                        AttrTemplateProps::show(ctx, ui);
                                    }
                                    Kind::ItemTemplate => {
                                        for elem in ctx.state.data.get_item_templates().iter() {
                                            if elem.id == *id {
                                                ectx.data_mut(|d| {
                                                    d.insert_temp(egui::Id::from(EXPLORE_ELEMENT), elem.clone())
                                                });
                                                break;
                                            }
                                        }
                                        ItemTemplateProps::show(ctx, ui);
                                    }
                                    _ => {}
                                }
                            }
                        });
                    });
                });
        });
    }
}
