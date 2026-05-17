use crate::{
    CogsApp,
    comps::AppComponent,
    explore::{ExploreCategory, ExploreKind, ExploreViewState, TemplateTypeFilter},
};
use cogs_shared::domain::model::{
    AccessLevel,
    meta::{AttrTemplate, ItemTemplate, Kind},
};
use egui::{Color32, CursorIcon, Rect, RichText, Sense, Ui};
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

            if matches!(ctx.state.explore.category, ExploreCategory::Security) {
                let access_levels = access_level_rows_filtered(ctx);
                show_access_levels(ctx, &ectx, ui, &access_levels);
                return;
            }

            let resize_stroke = egui::Stroke::new(1.0, ui.visuals().text_color().gamma_multiply(0.55));
            ui.visuals_mut().widgets.hovered.bg_fill = Color32::TRANSPARENT;
            ui.visuals_mut().widgets.hovered.bg_stroke = resize_stroke;
            ui.visuals_mut().widgets.active.bg_stroke = resize_stroke;

            let table = TableBuilder::new(ui)
                .id_salt("explore_table")
                .striped(false)
                .resizable(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto().at_least(40.0)) // type
                .column(Column::initial(w[1]).at_least(100.0).at_most(250.0)) // name
                .column(Column::remainder().at_least(w[2].max(100.0)).resizable(true)) // description
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
                ExploreCategory::Security => {}
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

fn show_access_levels(ctx: &mut CogsApp, ectx: &egui::Context, ui: &mut Ui, elems: &[AccessLevel]) {
    let row_hover_fill = ui.visuals().widgets.hovered.bg_fill;
    let resize_stroke = egui::Stroke::new(1.0, ui.visuals().text_color().gamma_multiply(0.55));
    let spacing = ui.spacing().item_spacing;
    let row_height = 20.0;
    let header_height = 20.0;
    let available_width = ui.available_width();
    let total_height = header_height + spacing.y + elems.len() as f32 * (row_height + spacing.y);

    let mut widths = ctx.state.explore.table_col_widths.unwrap_or([40.0, 120.0, 150.0]);
    widths[0] = widths[0].clamp(40.0, 150.0);
    widths[1] = widths[1].clamp(100.0, 350.0);

    ui.allocate_ui(egui::vec2(available_width, total_height), |ui| {
        let table_rect = ui.max_rect();
        let left = table_rect.left();
        let top = table_rect.top();
        let type_width = widths[0];
        let name_width = widths[1];
        let description_width = (available_width - type_width - name_width - spacing.x * 2.0).max(100.0);

        let type_x = left;
        let name_x = type_x + type_width + spacing.x;
        let description_x = name_x + name_width + spacing.x;

        paint_access_level_cell(
            ui,
            Rect::from_min_size(egui::pos2(type_x, top), egui::vec2(type_width, header_height)),
            |ui| {
                ui.label(RichText::new("type").color(Color32::GRAY));
            },
        );
        paint_access_level_cell(
            ui,
            Rect::from_min_size(egui::pos2(name_x, top), egui::vec2(name_width, header_height)),
            |ui| {
                ui.label(RichText::new("name").color(Color32::GRAY));
            },
        );
        paint_access_level_cell(
            ui,
            Rect::from_min_size(egui::pos2(description_x, top), egui::vec2(description_width, header_height)),
            |ui| {
                ui.label(RichText::new("description").color(Color32::GRAY));
            },
        );

        let mut row_top = top + header_height + spacing.y;
        for elem in elems {
            let row_rect = Rect::from_min_size(egui::pos2(left, row_top), egui::vec2(available_width, row_height));
            let row_hovered = ui.input(|i| i.pointer.hover_pos().is_some_and(|pos| row_rect.contains(pos)));

            if row_hovered {
                ui.painter().rect_filled(row_rect, egui::CornerRadius::ZERO, row_hover_fill);
                ectx.request_repaint();
            }

            paint_access_level_cell(
                ui,
                Rect::from_min_size(egui::pos2(type_x, row_top), egui::vec2(type_width, row_height)),
                |ui| {
                    ui.label(RichText::new("A.L.").color(Color32::GRAY))
                        .on_hover_text("Access Level")
                        .on_hover_cursor(CursorIcon::Help);
                },
            );
            paint_access_level_cell(
                ui,
                Rect::from_min_size(egui::pos2(name_x, row_top), egui::vec2(name_width, row_height)),
                |ui| {
                    ui.label(&elem.name);
                },
            );
            paint_access_level_cell(
                ui,
                Rect::from_min_size(egui::pos2(description_x, row_top), egui::vec2(description_width, row_height)),
                |ui| {
                    ui.label(RichText::new(elem.description.as_deref().unwrap_or_default()).color(Color32::GRAY));
                },
            );

            let row_resp = ui
                .interact(row_rect, ui.id().with(("access_level_row", &elem.id)), Sense::click())
                .on_hover_cursor(CursorIcon::PointingHand);

            if row_resp.double_clicked() {
                ctx.state
                    .explore
                    .open_windows_access_level
                    .insert(elem.id.clone(), elem.clone());
            }

            row_top += row_height + spacing.y;
        }

        resize_access_level_column(
            ui,
            "type",
            left + type_width + spacing.x * 0.5,
            table_rect,
            resize_stroke,
            |pointer_x| {
                widths[0] = (pointer_x - left).clamp(40.0, 150.0);
            },
        );
        resize_access_level_column(
            ui,
            "name",
            name_x + name_width + spacing.x * 0.5,
            table_rect,
            resize_stroke,
            |pointer_x| {
                widths[1] = (pointer_x - name_x).clamp(100.0, 350.0);
            },
        );
    });

    widths[2] = (available_width - widths[0] - widths[1] - spacing.x * 2.0).max(100.0);
    ctx.state.explore.table_col_widths = Some(widths);
}

fn paint_access_level_cell(ui: &mut Ui, rect: Rect, add_contents: impl FnOnce(&mut Ui)) {
    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
        |ui| {
            ui.set_clip_rect(rect);
            add_contents(ui);
        },
    );
}

fn resize_access_level_column(
    ui: &mut Ui,
    id_salt: &'static str,
    x: f32,
    table_rect: Rect,
    resize_stroke: egui::Stroke,
    update_width: impl FnOnce(f32),
) {
    let resize_rect = Rect::from_min_max(egui::pos2(x, table_rect.top()), egui::pos2(x, table_rect.bottom()))
        .expand(ui.style().interaction.resize_grab_radius_side);
    let response = ui.interact(
        resize_rect,
        ui.id().with(("access_level_resize", id_salt)),
        Sense::click_and_drag(),
    );

    if response.hovered() || response.dragged() {
        ui.set_cursor_icon(CursorIcon::ResizeColumn);
        ui.painter().line_segment(
            [egui::pos2(x, table_rect.top()), egui::pos2(x, table_rect.bottom())],
            resize_stroke,
        );
    }

    if response.dragged()
        && let Some(pointer) = ui.ctx().pointer_latest_pos()
    {
        update_width(pointer.x);
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
