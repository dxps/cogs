use crate::{
    CogsApp, SourceType,
    comps::{AppComponent, item::render_ask_window},
    constants::EXPLORE_ELEMENT,
};
use cogs_shared::domain::model::{
    Action, Id,
    meta::{Item, ItemTemplate},
};
use egui::{Align, Button, CursorIcon, Direction, Grid, Layout, Margin, Window, vec2};
use std::sync::{Arc, Mutex};

pub struct ItemWindow;

pub(super) struct ItemWindowState<'a> {
    ectx: &'a egui::Context,
    id: Id,
    act_id: egui::Id,
    action: Action,
    title: &'static str,

    /// An item can be created from scratch or from a template.
    /// The tuple is (src_type, item template (if src_type is Template), continue)
    new_item_src_type_tmpl_cont: Option<(Option<SourceType>, Option<ItemTemplate>, bool)>,
    // new_item_src_type_id: egui::Id,
    // new_item_src_tmpl_id: egui::Id,
}

impl<'a> ItemWindowState<'a> {
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

        let (src_type, src_tmpl, cont) = ectx.data(|d| {
            (
                d.get_temp::<Option<SourceType>>(egui::Id::from("new_item_src_type"))
                    .unwrap_or(None),
                d.get_temp::<Option<ItemTemplate>>(egui::Id::from("new_item_src_tmpl"))
                    .unwrap_or(None),
                d.get_temp(egui::Id::from("new_item_src_cont")).unwrap_or(false),
            )
        });

        let val = match (src_type, src_tmpl, cont) {
            (Some(st), None, c) => Some((Some(st), None, c)),
            (Some(st), Some(ste), c) => Some((Some(st), Some(ste), c)),
            _ => None,
        };

        log::debug!("[ItemWindowState::from_ctx] new_item_src_type_tmpl_cont={:#?}", val);

        Self {
            ectx,
            id,
            act_id,
            action,
            title,
            new_item_src_type_tmpl_cont: val,
        }
    }

    pub(super) fn new_item_src_type_tmpl_cont(&self) -> Option<(Option<SourceType>, Option<ItemTemplate>, bool)> {
        self.new_item_src_type_tmpl_cont.clone()
    }

    pub(super) fn set_new_item_src_type_tmpl_cont(&mut self, value: Option<(Option<SourceType>, Option<ItemTemplate>, bool)>) {
        self.new_item_src_type_tmpl_cont = value.clone();
        let (src_type, src_tmpl, cont) = match value {
            Some(v) => (v.0, v.1, v.2),
            None => (None, None, false),
        };
        self.ectx.data_mut(|d| {
            d.insert_temp::<Option<SourceType>>(egui::Id::from("new_item_src_type"), src_type);
            d.insert_temp::<Option<ItemTemplate>>(egui::Id::from("new_item_src_tmpl"), src_tmpl);
            d.insert_temp::<bool>(egui::Id::from("new_item_src_cont"), cont);
        });
    }
}

impl ItemWindow {
    fn render_header(ui: &mut egui::Ui, s: &ItemWindowState<'_>) {
        ui.horizontal(|ui| {
            ui.add_space(40.0);
            let w = ui.available_width() - 8.0; // right pad used in grid
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

    fn render_form_grid(ui: &mut egui::Ui, _ectx: &egui::Context, _element: &mut Item, s: &mut ItemWindowState<'_>) {
        ui.horizontal(|ui| {
            ui.add_space(14.0);
            Grid::new(format!("item_win_{}_grid", s.id))
                .spacing([10.0, 10.0])
                .num_columns(2)
                .show(ui, |_ui| {
                    // TODO: implement the item form fields.
                });
            ui.add_space(8.0);
        });
    }

    fn render_footer_buttons(
        app: &mut CogsApp,
        ui: &mut egui::Ui,
        ectx: &egui::Context,
        element: &mut Item,
        state: &mut ItemWindowState<'_>,
    ) {
        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
            ui.add_space(18.0);

            if state.action.is_view() {
                if ui.button("    Edit    ").on_hover_cursor(CursorIcon::PointingHand).clicked() {
                    ectx.data_mut(|d| d.insert_temp(state.act_id, Action::Edit));
                }
            } else {
                let enabled = false;
                let resp = ui
                    .add_enabled(enabled, Button::new("    Save    "))
                    .on_disabled_hover_text("Provide TBD ...");

                if resp.clicked() {
                    // TODO: save the item
                    cleanup(app, ectx, state);
                }
            }

            ui.add_space(8.0);

            if ui.button("  Cancel  ").on_hover_cursor(CursorIcon::PointingHand).clicked() {
                cleanup(app, ectx, state);
            }

            if !element.id.is_zero() {
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

        let binding = ectx
            .data(|d| d.get_temp::<Arc<Mutex<Item>>>(egui::Id::from(EXPLORE_ELEMENT)))
            .clone()
            .unwrap_or_default();

        let mut element = binding.lock().unwrap();
        let mut state = ItemWindowState::from_ctx(ectx, &element);

        let (_, _, cont) = match &state.new_item_src_type_tmpl_cont {
            Some((sty, ste, cont)) => (sty, ste, *cont),
            None => (&None, &None, false),
        };

        if element.id.is_zero() && !cont {
            render_ask_window(ctx, ectx, &mut state);
        } else {
            Window::new(format!("item_{}_win", element.id))
                .title_bar(false)
                .resizable(false)
                .fixed_size(vec2(320.0, 300.0))
                .frame(egui::Frame::window(&ectx.style()).inner_margin(Margin::ZERO))
                .show(ectx, |ui| {
                    ui.vertical(|ui| {
                        Self::render_header(ui, &state);
                        ui.add_space(20.0); // only the space you explicitly want
                        Self::render_form_grid(ui, ectx, &mut element, &mut state);
                        ui.add_space(20.0);
                        Self::render_footer_buttons(ctx, ui, ectx, &mut element, &mut state);
                        ui.add_space(10.0);
                    })
                    .response
                    .on_hover_cursor(CursorIcon::Grab);
                });
        }
    }
}

pub(super) fn cleanup<'a>(ctx: &mut CogsApp, ectx: &egui::Context, state: &mut ItemWindowState<'_>) {
    ctx.state.explore.open_windows_item.remove(&state.id);
    ectx.data_mut(|d| d.remove::<Action>(state.act_id));
    state.set_new_item_src_type_tmpl_cont(None);
}
