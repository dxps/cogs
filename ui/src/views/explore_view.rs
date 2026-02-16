use crate::{
    CogsApp,
    comps::{
        AppComponent, AttrTemplatePreview, AttrTemplateWindow, Dropdown, DropdownItem, DropdownStyle, ExploreTable, ItemTemplatePreview, ItemTemplateWindow, menu_row
    },
    constants::{EXPLORE_ELEMENT, ICON_ATTR_TMPL, ICON_HELP, ICON_ITEM, ICON_ITEM_TMPL, ICON_RARROW, ICON_TMPL, POPUP_ROW_WIDTH},
    views::AppView,
};
use cogs_shared::domain::model::{
    Id,
    meta::{AttrTemplate, ItemTemplate, Kind},
};
use const_format::concatcp;
use egui::{Color32, CursorIcon, Popup, RichText, Sense, Ui};
use egui_extras::{Size, Strip, StripBuilder};
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
    // Used when Category::Templates.
    TemplateType(TemplateTypeFilter),
    // Used when Category::Items.
    ItemTemplateId(Id),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TemplateTypeFilter {
    ItemTemplate,
    AttributeTemplate,
}

pub struct Explore {}

impl AppView for Explore {
    type Context = CogsApp;

    fn show(ctx: &mut Self::Context, ectx: &egui::Context) {
        //
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
                .size(Size::relative(0.6).at_least(500.0)) // Table cell.
                .size(Size::exact(20.0)) // middle (space) cell.
                .size(Size::remainder().at_least(80.0)) // Preview cell.
                .horizontal(|mut strip| {
                    show_table_cell(ctx, ectx, &mut strip);
                    strip.cell(|_| {}); // For that middle space.
                    show_preview_cell(ctx, ectx, &mut strip);
                    
                });
        });
    }
}

fn show_table_cell(ctx: &mut CogsApp, ectx: &egui::Context, strip: &mut Strip<'_, '_>) {
    // Keep `kind` valid whenever category changes or persisted state is stale.
    fn normalize_kind_for_category(ctx: &mut CogsApp) {
        match ctx.state.explore.category {
            ExploreCategory::Templates => match ctx.state.explore.kind {
                ExploreKind::All | ExploreKind::TemplateType(_) => {}
                ExploreKind::ItemTemplateId(_) => {
                    ctx.state.explore.kind = ExploreKind::All;
                }
            },
            ExploreCategory::Items => match ctx.state.explore.kind {
                ExploreKind::All | ExploreKind::ItemTemplateId(_) => {}
                ExploreKind::TemplateType(_) => {
                    ctx.state.explore.kind = ExploreKind::All;
                }
            },
        }
    }

    // Optional: clear current selection if it no longer passes active filter.
    fn selection_passes_filter(ctx: &CogsApp) -> bool {
        let Some((sel_kind, sel_id)) = &ctx.state.explore.curr_sel_elem else {
            return true;
        };

        match ctx.state.explore.category {
            ExploreCategory::Templates => match &ctx.state.explore.kind {
                ExploreKind::All => {
                    // Any template kind visible.
                    *sel_kind == Kind::ItemTemplate || *sel_kind == Kind::AttributeTemplate
                }
                ExploreKind::TemplateType(TemplateTypeFilter::ItemTemplate) => {
                    *sel_kind == Kind::ItemTemplate
                }
                ExploreKind::TemplateType(TemplateTypeFilter::AttributeTemplate) => {
                    *sel_kind == Kind::AttributeTemplate
                }
                ExploreKind::ItemTemplateId(_) => true,
            },

            ExploreCategory::Items => match &ctx.state.explore.kind {
                ExploreKind::All => {
                    // Any item visible in Items category.
                    // Assuming you store selected item as Kind::ItemTemplate? adjust if you have Kind::Item.
                    true
                }
                ExploreKind::ItemTemplateId(tpl_id) => {
                    // If selection points to an item, keep only if item's template matches tpl_id.
                    // TODO
                    // ctx.state
                    //     .data
                    //     .get_items() // TODO
                    //     .iter()
                    //     .find(|it| it.id == *sel_id)
                    //     .map(|it| it.template_id == *tpl_id) // <- replace field name if different
                    //     .unwrap_or(false)
                    true 
                }
                ExploreKind::TemplateType(_) => true, // normalized away above
            },
        }
    }

    strip.cell(|ui| {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                show_category(ctx, ui);
                ui.add_space(15.0);
                show_kind(ctx, ui);
                ui.add_space(15.0);
                show_add_menu(ctx, ui);
            });
        });

        // Ensure filter state coherence before rendering table.
        normalize_kind_for_category(ctx);

        // If current selection is not visible with current filters, clear it.
        if !selection_passes_filter(ctx) {
            ctx.state.explore.curr_sel_elem = None;
        }

        // Table is expected to read `ctx.state.explore.category` and `ctx.state.explore.kind`
        // and apply filtering internally.
        ExploreTable::show(ctx, ui);

        // Open windows (unchanged).
        for (_, element) in ctx.state.explore.open_windows_item_template.clone().iter() {
            ectx.data_mut(|d| d.insert_temp(egui::Id::from(EXPLORE_ELEMENT), element.clone()));
            ItemTemplateWindow::show(ctx, ui);
        }
        for (_, element) in ctx.state.explore.open_windows_attr_template.clone().iter() {
            ectx.data_mut(|d| d.insert_temp(egui::Id::from(EXPLORE_ELEMENT), element.clone()));
            AttrTemplateWindow::show(ctx, ui);
        }
    });

}

fn show_category(ctx: &mut CogsApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label("Category");

        let items = vec![
            DropdownItem::new("Items", ExploreCategory::Items),
            DropdownItem::new("Templates", ExploreCategory::Templates),
        ];

        if let Some(v) = Dropdown::show(
            ui,
            ui.id().with("explore_categ_popup"),
            &ctx.state.explore.category,
            &items,
            DropdownStyle {
                // min_width: 120.0,
                // max_width: Some(260.0), // optional guard, if layout is tight.
                ..Default::default()
            },
        ) {
            ctx.state.explore.category = v;
        }
    });
}

// ///////// kind related parts, which are conditional on the selected category. ///////////

/// #[derive(Clone)]
struct KindOption {
    label: String,
    value: ExploreKind,
    italic: bool,
}

fn build_kind_options(ctx: &CogsApp) -> Vec<KindOption> {
    let mut out = vec![KindOption {
        label: "all".to_string(),
        value: ExploreKind::All,
        italic: true,
    }];

    match ctx.state.explore.category {
        ExploreCategory::Templates => {
            out.push(KindOption {
                label: "Item Template".to_string(),
                value: ExploreKind::TemplateType(TemplateTypeFilter::ItemTemplate),
                italic: false,
            });
            out.push(KindOption {
                label: "Attribute Template".to_string(),
                value: ExploreKind::TemplateType(TemplateTypeFilter::AttributeTemplate),
                italic: false,
            });
        }
        ExploreCategory::Items => {
            for t in ctx.state.data.get_item_templates() {
                out.push(KindOption {
                    label: t.name.clone(), // or title/display_name
                    value: ExploreKind::ItemTemplateId(t.id.clone()),
                    italic: false,
                });
            }
        }
    }

    out
}


fn show_kind(ctx: &mut CogsApp, ui: &mut Ui) {
    ui.label("Kind");

    let kind_opts = build_kind_options(ctx);
    let dd_items: Vec<DropdownItem<ExploreKind>> = kind_opts
        .iter()
        .map(|k| DropdownItem {
            label: k.label.clone(),
            value: k.value.clone(),
            italic: k.italic,
        })
        .collect();

    // Keep current kind valid for current category/options
    if !dd_items.iter().any(|i| i.value == ctx.state.explore.kind) {
        ctx.state.explore.kind = ExploreKind::All;
    }

    if let Some(v) = Dropdown::show(
        ui,
        ui.id().with("explore_kind_popup"),
        &ctx.state.explore.kind,
        &dd_items,
        DropdownStyle::default(),
    ) {
        ctx.state.explore.kind = v;
    }

    ui.label(RichText::new(ICON_HELP).color(Color32::GRAY).size(10.0))
        .on_hover_text(
            "If category is:\n- 'Items', you may filter by item template.\n- 'Templates', you may filter by template type.",
        )
        .on_hover_cursor(CursorIcon::Help);
}


fn show_add_menu(ctx: &mut CogsApp, ui: &mut Ui) {
    //
    let btn = ui
        .button(" + ")
        .interact(Sense::click())
        .on_hover_text_at_pointer("Add an element")
        .on_hover_cursor(CursorIcon::PointingHand);

    let (bg_inactive, bg_hovered, bg_active, sel_bg) = {
        let v = ui.visuals();
        (
            v.widgets.inactive.bg_fill,
            v.widgets.hovered.bg_fill,
            v.widgets.active.bg_fill,
            v.selection.bg_fill,
        )
    };

    let mut popup_style: egui::Style = ui.style().as_ref().clone();
    popup_style.visuals.window_fill = bg_inactive;
    popup_style.visuals.panel_fill = bg_inactive;
    popup_style.visuals.extreme_bg_color = bg_inactive;
    popup_style.visuals.widgets.inactive.bg_fill = bg_inactive;
    popup_style.visuals.widgets.hovered.bg_fill = bg_hovered;
    popup_style.visuals.widgets.active.bg_fill = bg_active;
    popup_style.visuals.selection.bg_fill = sel_bg;

    Popup::menu(&btn)
        .id(egui::Id::new("explore_add_popup"))
        .style(popup_style)
        .gap(5.0)
        .show(|ui| {
            if menu_row(ui, concatcp!(ICON_ITEM, "   Item"), false, Some(115.0)).clicked() {
                // TODO: open item form.
                ui.close();
            }

            ui.separator();

            let templates_resp = menu_row(ui, concatcp!(ICON_TMPL, "   Templates  ", ICON_RARROW), false, Some(115.0));

            let submenu_open_id = ui.id().with("templates_submenu_open");
            let submenu_rect_id = ui.id().with("templates_submenu_rect");

            let mut submenu_open = ui.ctx().data(|d| d.get_temp::<bool>(submenu_open_id)).unwrap_or(false);

            // Open submenu on hover/click of parent row.
            if templates_resp.hovered() || templates_resp.clicked() {
                submenu_open = true;
            }

            // Desired submenu position (to the right of parent row).
            let submenu_pos = egui::pos2(templates_resp.rect.right() + 6.0, templates_resp.rect.top());

            // We keep last known submenu rect in temp memory.
            let mut submenu_rect = ui
                .ctx()
                .data(|d| d.get_temp::<egui::Rect>(submenu_rect_id))
                .unwrap_or_else(|| egui::Rect::from_min_size(submenu_pos, egui::vec2(POPUP_ROW_WIDTH, 1.0)));

            if submenu_open {
                let mut style: egui::Style = ui.style().as_ref().clone();
                style.visuals.window_fill = style.visuals.extreme_bg_color;
                let area_resp = egui::Area::new(ui.id().with("templates_submenu_area"))
                    .order(egui::Order::Foreground)
                    .fixed_pos(submenu_pos)
                    .show(ui.ctx(), |ui| {
                        egui::Frame::popup(&style).show(ui, |ui| {
                            ui.set_min_width(POPUP_ROW_WIDTH);

                            if menu_row(ui, concatcp!(ICON_ITEM_TMPL, "   Item Template"), false, Some(150.0)).clicked() {
                                ctx.state
                                    .explore
                                    .open_windows_item_template
                                    .insert(Id::default(), Arc::new(Mutex::new(ItemTemplate::default())));
                                ui.close();
                            }

                            if menu_row(ui, concatcp!(ICON_ATTR_TMPL, "   Attribute Template"), false, Some(150.0)).clicked() {
                                ctx.state
                                    .explore
                                    .open_windows_attr_template
                                    .insert(Id::default(), Arc::new(Mutex::new(AttrTemplate::default())));
                                ui.close();
                            }

                        });
                    });

                submenu_rect = area_resp.response.rect;
            }

            // A pointer-based close logic.
            let pointer_pos = ui.input(|i| i.pointer.hover_pos());
            let should_keep_open = if let Some(p) = pointer_pos {
                let parent_zone = templates_resp.rect.expand2(egui::vec2(2.0, 2.0)); // Parent row zone (slightly expanded).
                let submenu_zone = submenu_rect.expand2(egui::vec2(2.0, 2.0)); // Submenu zone (slightly expanded).

                // Corridor between parent and submenu to allow transit.
                let corridor = egui::Rect::from_min_max(
                    egui::pos2(parent_zone.right(), parent_zone.top().min(submenu_zone.top())),
                    egui::pos2(submenu_zone.left(), parent_zone.bottom().max(submenu_zone.bottom())),
                )
                .expand2(egui::vec2(2.0, 0.0));

                parent_zone.contains(p) || submenu_zone.contains(p) || corridor.contains(p)
            } else {
                false
            };

            if submenu_open && !should_keep_open {
                submenu_open = false;
            }

            ui.ctx().data_mut(|d| {
                d.insert_temp(submenu_open_id, submenu_open);
                d.insert_temp(submenu_rect_id, submenu_rect);
            });

            if submenu_open {
                ui.ctx().request_repaint();
            }
        });
}

fn show_preview_cell(ctx: &mut CogsApp, ectx: &egui::Context,  strip: &mut Strip<'_, '_>) {
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
                        AttrTemplatePreview::show(ctx, ui);
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
                        ItemTemplatePreview::show(ctx, ui);
                    }
                    _ => {}
                }
            }
        });
    });
}