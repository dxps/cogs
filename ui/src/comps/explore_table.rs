use crate::{CogsApp, comps::AppComponent};
use cogs_shared::domain::model::meta::Kind;
use egui::{Color32, CursorIcon, RichText, Sense, Ui};
use egui_extras::{Column, TableBody, TableBuilder};
use std::sync::{Arc, Mutex};

pub struct ExploreTable {}

impl AppComponent for ExploreTable {
    type Context = CogsApp;

    fn show(ctx: &mut Self::Context, ui: &mut Ui) {
        //
        if !ctx.state.data.has_fetched_all() {
            match ctx.state.explore.category {
                crate::views::ExploreCategory::Items => {
                    // TODO
                    // ctx.state.data_mgmt.get_all_items(ectx);
                }
                crate::views::ExploreCategory::Templates => {
                    ctx.state.data.fetch_all_attr_templates(ui.ctx(), ctx.sendr.clone());
                    ctx.state.data.fetch_all_item_templates(ui.ctx(), ctx.sendr.clone());
                    // TODO: fetch_all_link_templates
                }
            }
        }

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add_space(10.0);

            let available_height = ui.available_height();
            let table = TableBuilder::new(ui)
                .striped(true)
                .resizable(false)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto().at_least(50.0)) // type
                .column(Column::auto().at_least(200.0)) // name
                .column(Column::remainder()) // description
                .max_scroll_height(available_height)
                .sense(Sense::click());

            let table = table.header(20.0, |mut header| {
                header.col(|ui| {
                    ui.label(RichText::new("type").color(Color32::GRAY));
                });
                header.col(|ui| {
                    ui.label(RichText::new("name").color(Color32::GRAY));
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
                        show_attr_templates(ctx, &mut body);
                        show_item_templates(ctx, &mut body);
                    });
                }
            }
        });
    }
}

fn show_attr_templates(ctx: &mut CogsApp, body: &mut TableBody<'_>) {
    for elem in &ctx.state.data.get_attr_templates() {
        let mut open_win = false; // Open element's details in a new window (for view and edit).
        let mut show_right = false; // Show this element's details to the right (of this table).
        body.row(20.0, |mut row| {
            row.col(|ui| {
                ui.label(RichText::new("A.T.").color(Color32::GRAY))
                    .on_hover_text("Attribute Template")
                    .on_hover_cursor(CursorIcon::Help);
            });
            row.col(|ui| {
                let label = ui.label(format!("{}", elem.name)).on_hover_cursor(CursorIcon::PointingHand);
                if label.double_clicked() {
                    open_win = true;
                } else if label.clicked() {
                    show_right = true;
                };
            });
            row.col(|ui| {
                let label = ui
                    .label(RichText::new(format!("{}", elem.description)).color(Color32::GRAY))
                    .on_hover_cursor(CursorIcon::PointingHand);
                if label.clicked() {
                    show_right = true;
                };
                if label.double_clicked() {
                    open_win = true;
                }
            });

            row.response().on_hover_cursor(CursorIcon::PointingHand);
            if row.response().double_clicked() {
                open_win = true;
            }
            if row.response().clicked() {
                show_right = true;
            }
        });
        if open_win {
            ctx.state
                .explore
                .open_windows_attr_template
                .insert(elem.id.clone(), Arc::new(Mutex::new(elem.clone())));
        }
        if show_right {
            ctx.state.explore.curr_sel_elem = Some((Kind::AttributeTemplate, elem.id.clone()));
        }
    }
}

fn show_item_templates(ctx: &mut CogsApp, body: &mut TableBody<'_>) {
    for elem in &ctx.state.data.get_item_templates() {
        let mut open_win = false; // Open element's details in a new window (for view and edit).
        let mut show_right = false; // Show this element's details to the right (of this table).
        body.row(20.0, |mut row| {
            row.col(|ui| {
                ui.label(RichText::new("I.T.").color(Color32::GRAY))
                    .on_hover_text("Item Template")
                    .on_hover_cursor(CursorIcon::Help);
            });
            row.col(|ui| {
                let label = ui.label(format!("{}", elem.name)).on_hover_cursor(CursorIcon::PointingHand);
                if label.double_clicked() {
                    open_win = true;
                } else if label.clicked() {
                    show_right = true;
                };
            });
            row.col(|ui| {
                let label = ui
                    .label(RichText::new(format!("{}", elem.description)).color(Color32::GRAY))
                    .on_hover_cursor(CursorIcon::PointingHand);
                if label.clicked() {
                    show_right = true;
                };
                if label.double_clicked() {
                    open_win = true;
                }
            });

            row.response().on_hover_cursor(CursorIcon::PointingHand);
            if row.response().double_clicked() {
                open_win = true;
            }
            if row.response().clicked() {
                show_right = true;
            }
        });
        if open_win {
            ctx.state
                .explore
                .open_windows_item_template
                .insert(elem.id.clone(), Arc::new(Mutex::new(elem.clone())));
        }
        if show_right {
            ctx.state.explore.curr_sel_elem = Some((Kind::ItemTemplate, elem.id.clone()));
        }
    }
}
