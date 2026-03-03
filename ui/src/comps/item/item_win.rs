use crate::{
    CogsApp,
    comps::{
        AppComponent, AttrsLinksTab, horiz_tab,
        item::{render_add_attr, render_ask_window, render_attrs},
    },
    constants::EXPLORE_ELEMENT,
};
use cogs_shared::domain::model::{Action, Id, meta::Item};
use egui::{Align, Button, CursorIcon, Direction, Grid, Layout, Margin, Rect, Window};

pub struct ItemWindow;

pub(super) struct ItemWindowState {
    pub(super) id: Id,
    act_id: egui::Id,
    action: Action,
    title: &'static str,

    tab_id: egui::Id,
    tab: AttrsLinksTab,
}

impl<'a> ItemWindowState {
    fn from_ctx(ectx: &'a egui::Context, element: &Item) -> Self {
        let id = element.id.clone();
        let act_id = egui::Id::from(format!("item_id_{}_action", id));
        let action = if id.is_zero() {
            Action::Create
        } else {
            ectx.data(|d| d.get_temp::<Action>(act_id)).unwrap_or(Action::View)
        };
        let title = match action {
            Action::Create => "New Item",
            Action::Edit => "Edit Item",
            _ => "Item",
        };
        let tab_id = egui::Id::from(format!("item_form_{}_tab", id));
        let tab = ectx
            .data(|d| d.get_temp::<AttrsLinksTab>(tab_id))
            .unwrap_or(AttrsLinksTab::Attributes);

        Self {
            id,
            act_id,
            action,
            title,
            tab_id,
            tab,
        }
    }
}

impl ItemWindow {
    fn render_header(ui: &mut egui::Ui, s: &ItemWindowState) {
        ui.horizontal(|ui| {
            let w = ui.available_width();
            ui.allocate_ui_with_layout(
                egui::vec2(w.max(0.0), 0.0),
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    ui.add_enabled(false, egui::Label::new(egui::RichText::new(s.title).size(13.0)));
                    if !s.id.is_zero() {
                        ui.add_enabled(
                            s.action.is_edit(),
                            egui::Label::new(
                                egui::RichText::new(format!("(id: {})", s.id))
                                    .color(egui::Color32::GRAY)
                                    .size(10.0),
                            ),
                        );
                    }
                },
            );
        });
    }

    fn render_content(ctx: &mut CogsApp, ui: &mut egui::Ui, element: &mut Item, state: &mut ItemWindowState) {
        ui.vertical(|ui| {
            ui.add_space(14.0);
            Self::render_tabs(ui, state);
            ui.add_space(16.0);
            Grid::new(format!("item_win_{}_grid", state.id))
                .spacing([10.0, 10.0])
                .num_columns(2)
                .show(ui, |ui| {
                    // TODO: implement the item form fields.
                    match state.tab {
                        AttrsLinksTab::Attributes => {
                            render_attrs(ctx, ui, element, state);
                        }
                        AttrsLinksTab::Links => {}
                    }
                });
            if state.action != Action::View {
                ui.add_space(16.0);
                render_add_attr(ctx, ui, element, state);
            }
            ui.add_space(8.0);
        });
    }

    fn render_tabs(ui: &mut egui::Ui, s: &mut ItemWindowState) {
        ui.scope(|ui| {
            ui.spacing_mut().button_padding = egui::vec2(4.0, 2.0);

            let desired = egui::vec2(127.8, 21.7);
            let full = ui.max_rect();
            let x = full.center().x - desired.x * 0.5;
            let y = ui.cursor().min.y;
            let rect = Rect::from_min_size(egui::pos2(x, y), desired);

            ui.scope_builder(
                egui::UiBuilder::new()
                    .max_rect(rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
                |ui| {
                    let attrs_selected = s.tab == AttrsLinksTab::Attributes;
                    if horiz_tab(ui, "Attributes", attrs_selected).clicked() {
                        s.tab = AttrsLinksTab::Attributes;
                        ui.ctx().data_mut(|d| d.insert_temp(s.tab_id, s.tab));
                    }
                    let links_selected = s.tab == AttrsLinksTab::Links;
                    if horiz_tab(ui, "Links", links_selected).clicked() {
                        s.tab = AttrsLinksTab::Links;
                        ui.ctx().data_mut(|d| d.insert_temp(s.tab_id, s.tab));
                    }
                },
            );

            ui.advance_cursor_after_rect(rect);
        });
    }

    fn render_footer_buttons(
        app: &mut CogsApp,
        ui: &mut egui::Ui,
        ectx: &egui::Context,
        item: &mut Item,
        state: &mut ItemWindowState,
    ) {
        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
            ui.add_space(18.0);

            if state.action.is_view() {
                if ui.button("    Edit    ").on_hover_cursor(CursorIcon::PointingHand).clicked() {
                    ectx.data_mut(|d| d.insert_temp(state.act_id, Action::Edit));
                }
            } else {
                let enabled = item.has_attributes();
                let resp = ui
                    .add_enabled(enabled, Button::new("    Save    "))
                    .on_hover_cursor(CursorIcon::PointingHand)
                    .on_disabled_hover_text("Provide at least one attribute\nbefore saving the item.");

                if resp.clicked() {
                    // TODO: save the item
                    cleanup(app, ectx, state);
                }
            }

            ui.add_space(8.0);

            if ui.button("  Cancel  ").on_hover_cursor(CursorIcon::PointingHand).clicked() {
                cleanup(app, ectx, state);
            }

            if !item.id.is_zero() {
                ui.with_layout(
                    Layout::from_main_dir_and_cross_align(Direction::LeftToRight, Align::Min),
                    |ui| {
                        ui.add_space(18.0);
                        if ui.button("  Delete   ").on_hover_cursor(CursorIcon::PointingHand).clicked() {
                            // TODO: delete the item
                            cleanup(app, ectx, state);
                        }
                    },
                );
            }
        });
    }
}

impl AppComponent for ItemWindow {
    type Context = CogsApp;

    /// It shows the form for creating or editing an item template.
    /// In `ui.ctx().data` it expects an `Arc<Mutex<Item>>` under `EXPLORE_ELEMENT` id.
    fn show(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {
        let ectx = ui.ctx();

        let mut element = ectx
            .data(|d| d.get_temp::<Item>(egui::Id::from(EXPLORE_ELEMENT)))
            .clone()
            .unwrap_or_default();

        let mut state = ItemWindowState::from_ctx(ectx, &element);

        let (_, _, cont) = match &ctx.state.explore.add_item_src_type_tmpl_cont {
            Some((sty, ste, cont)) => (sty, ste, *cont),
            None => (&None, &None, false),
        };

        Window::new(format!("item_{}_win", element.id))
            .title_bar(false)
            .resizable(true)
            .min_width(400.0)
            .max_width(500.0)
            .min_height(200.0)
            .frame(egui::Frame::window(&ectx.style()).inner_margin(Margin::ZERO))
            .show(ectx, |ui| {
                ui.vertical(|ui| {
                    if element.id.is_zero() && !cont {
                        render_ask_window(ctx, ui, &mut state);
                    } else {
                        Self::render_header(ui, &state);
                        ui.add_space(10.0);
                        Self::render_content(ctx, ui, &mut element, &mut state);
                        ui.add_space(20.0);
                        Self::render_footer_buttons(ctx, ui, ectx, &mut element, &mut state);
                        ui.add_space(10.0);
                    }
                })
                .response
                .on_hover_cursor(CursorIcon::Grab);
            });
    }
}

pub(super) fn cleanup(ctx: &mut CogsApp, ectx: &egui::Context, state: &mut ItemWindowState) {
    ctx.state.explore.open_windows_item.remove(&state.id);
    log::debug!(
        "[cleanup] Updated open_windows_item: {:?}",
        ctx.state.explore.open_windows_item
    );
    ectx.data_mut(|d| d.remove::<Action>(state.act_id));
    ctx.state.explore.add_item_src_type_tmpl_cont = None;
}
