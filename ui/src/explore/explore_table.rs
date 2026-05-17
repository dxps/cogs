use crate::{
    CogsApp,
    comps::AppComponent,
    explore::{ExploreCategory, ExploreKind, ExploreViewState, TemplateTypeFilter},
};
use cogs_shared::domain::model::{
    AccessLevel,
    meta::{AttrTemplate, ItemTemplate, Kind},
};
use egui::{Color32, CursorIcon, RichText, Sense, Ui};
use egui_extras::{Column, TableBody, TableBuilder};
use std::sync::{Arc, Mutex};

pub struct ExploreTable {}

impl AppComponent for ExploreTable {
    type Context = CogsApp;

    fn show(ctx: &mut Self::Context, ui: &mut Ui) {
        // Fetch data.
        match ctx.state.explore.category {
            ExploreCategory::Items => {
                // TODO: fetch items when implemented.
                // ctx.state.data.fetch_all_items(ui.ctx(), ctx.sendr.clone());
            }
            ExploreCategory::Templates => {
                if !ctx.state.data.has_fetched_all() {
                    ctx.state.data.fetch_all_attr_templates(ui.ctx(), ctx.sendr.clone());
                    ctx.state.data.fetch_all_item_templates(ui.ctx(), ctx.sendr.clone());
                    // TODO: fetch_all_link_templates
                }
            }
            ExploreCategory::Security => {
                if !ctx.state.data.has_fetched_access_levels() {
                    ctx.state.data.fetch_all_access_levels(ui.ctx(), ctx.sendr.clone());
                }
            }
        }

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add_space(10.0);

            let ectx = ui.ctx().clone();
            let available_height = ui.available_height();
            let w = ctx.state.explore.table_col_widths.unwrap_or([40.0, 120.0, 150.0]);

            let table = TableBuilder::new(ui)
                .id_salt("explore_table")
                .striped(true)
                .resizable(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto().at_least(40.0)) // type
                .column(Column::initial(w[1]).at_least(100.0).at_most(250.0)) // name
                .column(Column::initial(w[2]).at_least(100.0).at_most(400.0)) // description
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

            let remember_widths = |body: &TableBody<'_>, explore: &mut ExploreViewState| {
                let widths = body.widths();
                if widths.len() >= 3 {
                    explore.table_col_widths = Some([widths[0], widths[1], widths[2]]);
                }
            };

            match ctx.state.explore.category {
                ExploreCategory::Items => {
                    table.body(|body| {
                        // todo: render items rows here.
                        remember_widths(&body, &mut ctx.state.explore);
                    });
                }
                ExploreCategory::Templates => {
                    let (item_templates, attr_templates) = template_rows_filtered(ctx);

                    table.body(|mut body| {
                        show_attr_templates(ctx, &mut body, &attr_templates);
                        show_item_templates(ctx, &mut body, &item_templates);

                        remember_widths(&body, &mut ctx.state.explore);
                    });
                }
                ExploreCategory::Security => {
                    let access_levels = access_level_rows_filtered(ctx);

                    table.body(|mut body| {
                        show_access_levels(ctx, &ectx, &mut body, &access_levels);

                        remember_widths(&body, &mut ctx.state.explore);
                    });
                }
            }
        });
    }
}

fn access_level_rows_filtered(ctx: &CogsApp) -> Vec<AccessLevel> {
    match (&ctx.state.explore.category, &ctx.state.explore.kind) {
        (ExploreCategory::Security, ExploreKind::All | ExploreKind::AccessLevel) => ctx.state.data.get_access_levels().to_vec(),
        (ExploreCategory::Security, _) => Vec::new(),
        _ => Vec::new(),
    }
}

fn template_rows_filtered(ctx: &CogsApp) -> (Vec<ItemTemplate>, Vec<AttrTemplate>) {
    match (&ctx.state.explore.category, &ctx.state.explore.kind) {
        // Category = Templates, Kind = all
        (ExploreCategory::Templates, ExploreKind::All) => (
            ctx.state.data.get_item_templates().to_vec(),
            ctx.state.data.get_attr_templates().to_vec(),
        ),

        // Category = Templates, Kind = Item Template
        (ExploreCategory::Templates, ExploreKind::TemplateType(TemplateTypeFilter::ItemTemplate)) => {
            (ctx.state.data.get_item_templates().to_vec(), Vec::new())
        }

        // Category = Templates, Kind = Attribute Template
        (ExploreCategory::Templates, ExploreKind::TemplateType(TemplateTypeFilter::AttributeTemplate)) => {
            (Vec::new(), ctx.state.data.get_attr_templates().to_vec())
        }

        // Stale value (from Items mode) while in Templates: fallback to All.
        (ExploreCategory::Templates, ExploreKind::ItemTemplateId(_) | ExploreKind::AccessLevel) => (
            ctx.state.data.get_item_templates().to_vec(),
            ctx.state.data.get_attr_templates().to_vec(),
        ),

        // Category = Items -> templates list isn't relevant here.
        (ExploreCategory::Items, _) => (Vec::new(), Vec::new()),

        // Category = Security -> templates list isn't relevant here.
        (ExploreCategory::Security, _) => (Vec::new(), Vec::new()),
    }
}

fn show_access_levels(ctx: &mut CogsApp, ectx: &egui::Context, body: &mut TableBody<'_>, elems: &[AccessLevel]) {
    for elem in elems {
        let mut open_win = false;
        let hovered_id = egui::Id::new(("access_level_row_hovered", &elem.id));
        body.row(20.0, |mut row| {
            let was_hovered = ectx.data(|d| d.get_temp::<bool>(hovered_id)).unwrap_or(false);
            row.set_hovered(was_hovered);

            let mut is_hovered = false;

            row.col(|ui| {
                ui.label(RichText::new("A.L.").color(Color32::GRAY))
                    .on_hover_text("Access Level")
                    .on_hover_cursor(CursorIcon::Help);
            });

            row.col(|ui| {
                let resp = ui.label(&elem.name).on_hover_cursor(CursorIcon::PointingHand);
                is_hovered |= resp.hovered();
            });

            row.col(|ui| {
                let resp = ui
                    .label(RichText::new(elem.description.as_deref().unwrap_or_default()).color(Color32::GRAY))
                    .on_hover_cursor(CursorIcon::PointingHand);
                is_hovered |= resp.hovered();
            });

            let row_resp = row.response().on_hover_cursor(CursorIcon::PointingHand);
            is_hovered |= row_resp.hovered();
            if row_resp.double_clicked() {
                open_win = true;
            }

            ectx.data_mut(|d| d.insert_temp(hovered_id, is_hovered));
            if is_hovered {
                ectx.request_repaint();
            }
        });

        if open_win {
            ctx.state
                .explore
                .open_windows_access_level
                .insert(elem.id.clone(), elem.clone());
        }
    }
}

fn show_attr_templates(ctx: &mut CogsApp, body: &mut TableBody<'_>, elems: &[AttrTemplate]) {
    for elem in elems {
        let mut open_win = false;
        let mut show_right = false;

        body.row(20.0, |mut row| {
            row.col(|ui| {
                ui.label(RichText::new("A.T.").color(Color32::GRAY))
                    .on_hover_text("Attribute Template")
                    .on_hover_cursor(CursorIcon::Help);
            });

            row.col(|ui| {
                let label = ui.label(&elem.name).on_hover_cursor(CursorIcon::PointingHand);

                if label.double_clicked() {
                    open_win = true;
                } else if label.clicked() {
                    show_right = true;
                }
            });

            row.col(|ui| {
                let label = ui
                    .label(RichText::new(&elem.description).color(Color32::GRAY))
                    .on_hover_cursor(CursorIcon::PointingHand);

                if label.double_clicked() {
                    open_win = true;
                } else if label.clicked() {
                    show_right = true;
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

fn show_item_templates(ctx: &mut CogsApp, body: &mut TableBody<'_>, elems: &[ItemTemplate]) {
    for elem in elems {
        let mut open_win = false;
        let mut show_right = false;

        body.row(20.0, |mut row| {
            row.col(|ui| {
                ui.label(RichText::new("I.T.").color(Color32::GRAY))
                    .on_hover_text("Item Template")
                    .on_hover_cursor(CursorIcon::Help);
            });

            row.col(|ui| {
                let label = ui.label(&elem.name).on_hover_cursor(CursorIcon::PointingHand);

                if label.double_clicked() {
                    open_win = true;
                } else if label.clicked() {
                    show_right = true;
                }
            });

            row.col(|ui| {
                let label = ui
                    .label(RichText::new(&elem.description).color(Color32::GRAY))
                    .on_hover_cursor(CursorIcon::PointingHand);

                if label.double_clicked() {
                    open_win = true;
                } else if label.clicked() {
                    show_right = true;
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
