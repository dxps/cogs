use crate::{
    CogsApp,
    comps::{
        AppComponent, AttrTemplateWindow, AttrTemplateProps, ExploreTable, ItemTemplateWindow, ItemTemplateProps, LinkTemplateForm,
        menu_row, paint_combo_chevron,
    },
    constants::{CORNER_RADIUS, EXPLORE_ELEMENT, ICON_HELP, ICON_SETTINGS, POPUP_ROW_WIDTH},
    views::AppView,
};
use cogs_shared::domain::model::{
    Id,
    meta::{AttrTemplate, ItemTemplate, Kind, LinkTemplate},
};
use const_format::concatcp;
use egui::{Color32, CursorIcon, Popup, RichText, Sense, TextStyle, TextWrapMode, Ui, WidgetText, pos2};
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
    Attribute,
    Link,
    Item,
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
    strip.cell(|ui| {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                show_category(ctx, ui);
                ui.add_space(15.0);
                show_kind(ctx, ui);
                ui.add_space(15.0);
                // The "+" button and its menu.
                show_add_menu(ctx, ui);
            })
        });

        ExploreTable::show(ctx, ui);

        for (_, element) in ctx.state.explore.open_windows_item_template.clone().iter() {
            ectx.data_mut(|d| d.insert_temp(egui::Id::from(EXPLORE_ELEMENT), element.clone()));
            ItemTemplateWindow::show(ctx, ui);
        }
        for (_, element) in ctx.state.explore.open_windows_attr_template.clone().iter() {
            ectx.data_mut(|d| d.insert_temp(egui::Id::from(EXPLORE_ELEMENT), element.clone()));
            AttrTemplateWindow::show(ctx, ui);
        }
        for (_, element) in ctx.state.explore.open_windows_link_template.clone().iter() {
            ectx.data_mut(|d| d.insert_temp(egui::Id::from(EXPLORE_ELEMENT), element.clone()));
            LinkTemplateForm::show(ctx, ui);
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
}

fn show_category(ctx: &mut CogsApp, ui: &mut egui::Ui) {
    //
    ui.horizontal(|ui| {
        ui.label("Category:");

        let selected = match ctx.state.explore.category {
            ExploreCategory::Items => "Items",
            ExploreCategory::Templates => "Templates",
        };

        // Exact-size trigger rect.
        let h = 20.0;
        let w = 120.0;
        let (rect, resp) = ui.allocate_exact_size(egui::vec2(w, h), egui::Sense::click());

        let bg = if resp.is_pointer_button_down_on() {
            ui.visuals().widgets.active.bg_fill
        } else if resp.hovered() {
            ui.visuals().widgets.hovered.bg_fill
        } else {
            // This matches ComboBox-like resting background best in most themes.
            ui.visuals().widgets.inactive.bg_fill
            // For an even closer to popup/button body in your theme.
            // ui.visuals().widgets.open.bg_fill
        };

        ui.painter().rect_filled(rect, CORNER_RADIUS, bg);

        // Show the text w/ the selected category.
        ui.painter().text(
            egui::pos2(rect.left() + 10.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            selected,
            egui::TextStyle::Button.resolve(ui.style()),
            ui.visuals().text_color(),
        );

        // Show the chevron (down arrow) symbol.
        paint_combo_chevron(ui, rect);

        let resp = resp.on_hover_cursor(egui::CursorIcon::PointingHand);

        // Popup style that follows current theme (FRAPPE/LATTE).
        let mut popup_style: egui::Style = ui.style().as_ref().clone();
        let v = ui.visuals();

        // Match popup background to the same family as the trigger (resp) background.
        popup_style.visuals.window_fill = v.widgets.inactive.bg_fill;
        popup_style.visuals.panel_fill = v.widgets.inactive.bg_fill;
        popup_style.visuals.extreme_bg_color = v.widgets.inactive.bg_fill;

        // Optional: keep row hover/active consistent with current theme.
        popup_style.visuals.widgets.inactive.bg_fill = v.widgets.inactive.bg_fill;
        popup_style.visuals.widgets.hovered.bg_fill = v.widgets.hovered.bg_fill;
        popup_style.visuals.widgets.active.bg_fill = v.widgets.active.bg_fill;
        popup_style.visuals.selection.bg_fill = v.selection.bg_fill;

        Popup::menu(&resp)
            .id(ui.id().with("explore_categ_popup"))
            .style(popup_style)
            .gap(4.0)
            .show(|ui| {
                if menu_row(ui, "Items", false, Some(100.0)).clicked() {
                    ctx.state.explore.category = ExploreCategory::Items;
                    ui.close();
                }
                if menu_row(ui, "Templates", false, Some(100.0)).clicked() {
                    ctx.state.explore.category = ExploreCategory::Templates;
                    ui.close();
                }
            });
    });
}

fn show_kind(ctx: &mut CogsApp, ui: &mut Ui) {
    //
    ui.label("Kind:");

    let selected_text = match ctx.state.explore.kind {
        ExploreKind::All => "all",
        ExploreKind::Item => "Item",
        ExploreKind::Attribute => "Attribute",
        ExploreKind::Link => "Link",
    };
    let is_all = ctx.state.explore.kind == ExploreKind::All;

    let h = 20.0;
    let w = 120.0;
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(w, h), Sense::click());

    let (bg_inactive, bg_hovered, bg_active, sel_bg, text_inactive) = {
        let v = ui.visuals();
        (
            v.widgets.inactive.bg_fill,
            v.widgets.hovered.bg_fill,
            v.widgets.active.bg_fill,
            v.selection.bg_fill,
            v.widgets.inactive.fg_stroke.color,
        )
    };

    let bg = if resp.is_pointer_button_down_on() {
        bg_active
    } else if resp.hovered() {
        bg_hovered
    } else {
        bg_inactive
    };
    ui.painter().rect_filled(rect, CORNER_RADIUS, bg);

    // Paint selected text inside rect (no inner widget / no extra layout effects)
    let mut rt = RichText::new(selected_text).color(text_inactive);
    if is_all {
        rt = rt.italics();
    }
    let galley = WidgetText::from(rt).into_galley(ui, Some(TextWrapMode::Extend), f32::INFINITY, TextStyle::Button);
    let text_pos = pos2(rect.left() + 10.0, rect.center().y - galley.size().y * 0.5);
    ui.painter().galley(text_pos, galley, text_inactive);

    paint_combo_chevron(ui, rect);

    let resp = resp.on_hover_cursor(CursorIcon::PointingHand);

    let mut popup_style: egui::Style = ui.style().as_ref().clone();
    popup_style.visuals.window_fill = bg_inactive;
    popup_style.visuals.panel_fill = bg_inactive;
    popup_style.visuals.extreme_bg_color = bg_inactive;
    popup_style.visuals.widgets.inactive.bg_fill = bg_inactive;
    popup_style.visuals.widgets.hovered.bg_fill = bg_hovered;
    popup_style.visuals.widgets.active.bg_fill = bg_active;
    popup_style.visuals.selection.bg_fill = sel_bg;

    Popup::menu(&resp)
        .id(ui.id().with("explore_kind_popup"))
        .style(popup_style)
        .gap(4.0)
        .show(|ui| {
            if menu_row(ui, "all", true, Some(120.0)).clicked() {
                ctx.state.explore.kind = ExploreKind::All;
                ui.close();
            }

            if ctx.state.explore.category == ExploreCategory::Templates {
                if menu_row(ui, "Item", false, Some(120.0)).clicked() {
                    ctx.state.explore.kind = ExploreKind::Item;
                    ui.close();
                }
                if menu_row(ui, "Attribute", false, Some(120.0)).clicked() {
                    ctx.state.explore.kind = ExploreKind::Attribute;
                    ui.close();
                }
                if menu_row(ui, "Link", false, Some(120.0)).clicked() {
                    ctx.state.explore.kind = ExploreKind::Link;
                    ui.close();
                }
            }
        });

    ui.label(RichText::new(ICON_HELP).color(Color32::GRAY).size(10.0))
        .on_hover_text(
            "If category is 'Items', you may filter by their templates.\nIf category is 'Templates', you may filter by their types.",
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
            if menu_row(ui, concatcp!(ICON_SETTINGS, "  Item"), false, None).clicked() {
                // TODO: open item form.
                ui.close();
            }

            ui.separator();

            let templates_resp = menu_row(ui, "  Templates  >", false, None);

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

                            if menu_row(ui, "  Item Template", false, Some(POPUP_ROW_WIDTH)).clicked() {
                                ctx.state
                                    .explore
                                    .open_windows_item_template
                                    .insert(Id::default(), Arc::new(Mutex::new(ItemTemplate::default())));
                                ui.close();
                            }

                            if menu_row(ui, "  Attribute Template", false, Some(POPUP_ROW_WIDTH)).clicked() {
                                ctx.state
                                    .explore
                                    .open_windows_attr_template
                                    .insert(Id::default(), Arc::new(Mutex::new(AttrTemplate::default())));
                                ui.close();
                            }

                            if menu_row(ui, "  Link Template", false, Some(POPUP_ROW_WIDTH)).clicked() {
                                ctx.state
                                    .explore
                                    .open_windows_link_template
                                    .insert(Id::default(), Arc::new(Mutex::new(LinkTemplate::default())));
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
