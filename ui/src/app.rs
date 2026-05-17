use crate::{
    colors::FADED_COLOR,
    comps::{AppComponent, Footer, Header},
    constants::{APP_KEY, CORNER_RADIUS},
    explore::ExploreView,
    handle_msg,
    messages::UiMessage,
    state::UiState,
    views::{AppView, HomeView, LoginView, SettingsView, ViewName},
};
use cogs_shared::domain::model::meta::Kind;
use egui::{
    FontData,
    epaint::text::{FontInsert, InsertFontFamily},
};
use std::sync::mpsc::{Receiver, Sender, channel};

#[derive(Clone, Copy, PartialEq, Eq)]
struct CatppuccinTheme {
    rosewater: egui::Color32,
    maroon: egui::Color32,
    peach: egui::Color32,
    blue: egui::Color32,
    text: egui::Color32,
    overlay1: egui::Color32,
    surface2: egui::Color32,
    surface1: egui::Color32,
    surface0: egui::Color32,
    base: egui::Color32,
    mantle: egui::Color32,
    crust: egui::Color32,
}

const CATPPUCCIN_LATTE: CatppuccinTheme = CatppuccinTheme {
    rosewater: egui::Color32::from_rgb(220, 138, 120),
    maroon: egui::Color32::from_rgb(230, 69, 83),
    peach: egui::Color32::from_rgb(254, 100, 11),
    blue: egui::Color32::from_rgb(30, 102, 245),
    text: egui::Color32::from_rgb(76, 79, 105),
    overlay1: egui::Color32::from_rgb(140, 143, 161),
    surface2: egui::Color32::from_rgb(172, 176, 190),
    surface1: egui::Color32::from_rgb(188, 192, 204),
    surface0: egui::Color32::from_rgb(204, 208, 218),
    base: egui::Color32::from_rgb(239, 241, 245),
    mantle: egui::Color32::from_rgb(230, 233, 239),
    crust: egui::Color32::from_rgb(220, 224, 232),
};

const CATPPUCCIN_FRAPPE: CatppuccinTheme = CatppuccinTheme {
    rosewater: egui::Color32::from_rgb(242, 213, 207),
    maroon: egui::Color32::from_rgb(234, 153, 156),
    peach: egui::Color32::from_rgb(239, 159, 118),
    blue: egui::Color32::from_rgb(140, 170, 238),
    text: egui::Color32::from_rgb(198, 208, 245),
    overlay1: egui::Color32::from_rgb(131, 139, 167),
    surface2: egui::Color32::from_rgb(98, 104, 128),
    surface1: egui::Color32::from_rgb(81, 87, 109),
    surface0: egui::Color32::from_rgb(65, 69, 89),
    base: egui::Color32::from_rgb(48, 52, 70),
    mantle: egui::Color32::from_rgb(41, 44, 60),
    crust: egui::Color32::from_rgb(35, 38, 52),
};

#[derive(serde::Deserialize, serde::Serialize)] // So we can persist ui state on app shutdown.
#[serde(default)] // If we add new fields, give them default values when deserializing old state.
pub struct CogsApp {
    pub(crate) state: UiState,

    #[serde(skip)]
    /// Sender of UI messages.
    pub sendr: Sender<UiMessage>,

    #[serde(skip)]
    /// Receiver of UI messages.
    pub recvr: Receiver<UiMessage>,
}

impl Default for CogsApp {
    fn default() -> Self {
        let (sendr, recvr) = channel();
        Self {
            state: UiState::default(),
            sendr,
            recvr,
        }
    }
}

impl CogsApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        let ectx = &cc.egui_ctx;
        Self::init_font(ectx);

        // Image loading init.
        egui_extras::install_image_loaders(ectx);

        // Zoom setting.
        ectx.set_zoom_factor(1.26);

        ui_init_cosmetics(ectx);

        // Load previous app state, if any.
        // Note: The `persistence` feature must be enabled for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }

    fn init_font(ctx: &egui::Context) {
        ctx.add_font(FontInsert::new(
            "Supreme",
            FontData::from_static(include_bytes!("../assets/fonts/Supreme-Regular-icons.ttf")),
            vec![
                InsertFontFamily {
                    family: egui::FontFamily::Proportional,
                    priority: egui::epaint::text::FontPriority::Highest,
                },
                InsertFontFamily {
                    family: egui::FontFamily::Monospace,
                    priority: egui::epaint::text::FontPriority::Lowest,
                },
            ],
        ));
    }

    /// Further app init when running on web.
    #[cfg(target_arch = "wasm32")]
    pub fn init_web(&mut self, cc: &eframe::CreationContext<'_>) {
        let web_info = &cc.integration_info.web_info;
        log::trace!("[init web] webinfo: {:#?}", web_info);
        match web_info.location.hash.as_str() {
            "#/explore" => {
                self.state.set_curr_view(ViewName::Explore);
            }
            "#/login" => {
                self.state.set_curr_view(ViewName::Login);
            }
            _ => {
                self.state.set_curr_view(ViewName::Home);
            }
        }
    }
}

impl eframe::App for CogsApp {
    //
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        //
        let ectx = ui.ctx().clone();

        // State related logistics.
        let curr_theme = ectx.theme();
        if self.state.ui_theme != curr_theme {
            self.state.ui_theme = curr_theme;
            ui_init_cosmetics(&ectx);
        }

        Header::show(self, ui);

        if let Ok(res) = self.recvr.try_recv() {
            log::trace!("Received {:?}", res);
            match res {
                UiMessage::Login(data) => match data {
                    Ok(acc_sess) => match acc_sess {
                        Some((account, session)) => {
                            self.state.auth.user_account = Some(account);
                            self.state.auth.user_session = Some(session);
                            self.state.auth.login_error = None;
                            self.state.set_curr_view(ViewName::Home);
                        }
                        None => {
                            self.state.auth.login_error = None;
                            self.state.auth.user_account = None;
                        }
                    },
                    Err(err) => {
                        self.state.auth.login_error = Some(err);
                    }
                },

                UiMessage::Logout => {
                    handle_msg(UiMessage::Logout, &self.state);
                    self.state.auth.user_account = None;
                    self.state.auth.user_session = None;
                    self.state.set_curr_view(ViewName::Home);
                }

                UiMessage::Settings => {}

                UiMessage::AttrTemplateUpserted(_) => {
                    self.state.data.fetch_all_attr_templates(&ectx, self.sendr.clone());
                    ectx.request_repaint();
                }

                UiMessage::AttrTemplateDeleted(_) => {
                    self.state.data.fetch_all_attr_templates(&ectx, self.sendr.clone());
                    ectx.request_repaint();
                }

                UiMessage::ElementCreated(kind, ars) => {
                    match ars {
                        Ok(_id) => match kind {
                            Kind::Item => todo!(),
                            Kind::ItemTemplate => {
                                self.state.data.fetch_all_item_templates(&ectx, self.sendr.clone());
                                ectx.request_repaint();
                            }
                            Kind::AttributeTemplate => {
                                self.state.data.fetch_all_attr_templates(&ectx, self.sendr.clone());
                                ectx.request_repaint();
                            }
                            Kind::LinkTemplate => todo!(),
                        },
                        Err(_err) => {
                            // TODO: show a popup window.
                        }
                    }
                }

                UiMessage::ElementUpdated(kind, ar) => match ar {
                    Ok(_id) => match kind {
                        Kind::Item => todo!(),
                        Kind::ItemTemplate => todo!(),
                        Kind::AttributeTemplate => {
                            self.state.data.fetch_all_attr_templates(&ectx, self.sendr.clone());
                            ectx.request_repaint();
                        }
                        Kind::LinkTemplate => todo!(),
                    },
                    Err(_) => {
                        todo!();
                    }
                },

                UiMessage::ElementDeleted(kind, ar) => match ar {
                    Ok(_id) => match kind {
                        Kind::Item => todo!(),
                        Kind::ItemTemplate => {
                            self.state.data.fetch_all_item_templates(&ectx, self.sendr.clone());
                            ectx.request_repaint();
                        }
                        Kind::AttributeTemplate => {
                            self.state.data.fetch_all_attr_templates(&ectx, self.sendr.clone());
                            ectx.request_repaint();
                        }
                        Kind::LinkTemplate => todo!(),
                    },
                    Err(_) => todo!(),
                },

                UiMessage::AttrTemplatesFetched(data) => match data {
                    Ok(attr_templates) => {
                        self.state.data.set_attr_templates(attr_templates);
                    }
                    Err(err) => {
                        log::error!("[app.update] Error fetching attr templates: {}", err);
                    }
                },

                UiMessage::ItemTemplatesFetched(data) => match data {
                    Ok(data) => {
                        self.state.data.set_item_templates(data);
                    }
                    Err(err) => {
                        log::error!("[app.update] Error fetching item templates: {}", err);
                    }
                },

                UiMessage::AccessLevelsFetched(data) => match data {
                    Ok(data) => {
                        self.state.data.set_access_levels(data);
                    }
                    Err(err) => {
                        log::error!("[app.update] Error fetching access levels: {}", err);
                    }
                },
            }
        }

        egui::Panel::bottom("footer_panel")
            .show_separator_line(false)
            .show_inside(ui, |ui| {
                Footer::show(self, ui);
            });

        match self.state.curr_view() {
            ViewName::Home => HomeView::show(self, ui),
            ViewName::Explore => ExploreView::show(self, ui),
            ViewName::Settings => SettingsView::show(self, ui),
            ViewName::Login => {
                self.state.set_curr_view(ViewName::Login);
                LoginView::show(self, ui);
            }
        }
    }
}

fn ui_init_cosmetics(ctx: &egui::Context) {
    let theme = match ctx.theme() {
        egui::Theme::Light => CATPPUCCIN_LATTE,
        egui::Theme::Dark => CATPPUCCIN_FRAPPE,
    };

    ctx.global_style_mut(|style| {
        apply_catppuccin_theme(style, theme);

        style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::NONE;

        let r = egui::CornerRadius::same(CORNER_RADIUS as u8);
        style.visuals.widgets.inactive.corner_radius = r;
        style.visuals.widgets.hovered.corner_radius = r;
        style.visuals.widgets.active.corner_radius = r;
        style.visuals.widgets.open.corner_radius = r;
        style.visuals.widgets.noninteractive.corner_radius = r;

        // Centered shadow on windows.
        let alpha = if style.visuals.dark_mode { 75 } else { 40 };
        style.visuals.window_shadow = egui::epaint::Shadow {
            offset: [0, 0],
            blur: 25,
            spread: 5,
            color: egui::Color32::from_black_alpha(alpha),
        };

        _ = FADED_COLOR.set(style.visuals.text_color().gamma_multiply(0.6));
    });
}

fn apply_catppuccin_theme(style: &mut egui::Style, theme: CatppuccinTheme) {
    let old = style.visuals.clone();
    let is_latte = theme == CATPPUCCIN_LATTE;
    let shadow_color = if is_latte {
        egui::Color32::from_black_alpha(25)
    } else {
        egui::Color32::from_black_alpha(96)
    };

    style.visuals = egui::Visuals {
        hyperlink_color: theme.rosewater,
        faint_bg_color: theme.surface0,
        extreme_bg_color: theme.crust,
        code_bg_color: theme.mantle,
        warn_fg_color: theme.peach,
        error_fg_color: theme.maroon,
        window_fill: theme.base,
        panel_fill: theme.base,
        window_stroke: egui::Stroke {
            color: theme.overlay1,
            ..old.window_stroke
        },
        widgets: egui::style::Widgets {
            noninteractive: catppuccin_widget_visual(old.widgets.noninteractive, theme, theme.base),
            inactive: catppuccin_widget_visual(old.widgets.inactive, theme, theme.surface0),
            hovered: catppuccin_widget_visual(old.widgets.hovered, theme, theme.surface2),
            active: catppuccin_widget_visual(old.widgets.active, theme, theme.surface1),
            open: catppuccin_widget_visual(old.widgets.open, theme, theme.surface0),
        },
        selection: egui::style::Selection {
            bg_fill: theme.blue.linear_multiply(if is_latte { 0.4 } else { 0.2 }),
            stroke: egui::Stroke {
                color: theme.text,
                ..old.selection.stroke
            },
        },
        window_shadow: egui::epaint::Shadow {
            color: shadow_color,
            ..old.window_shadow
        },
        popup_shadow: egui::epaint::Shadow {
            color: shadow_color,
            ..old.popup_shadow
        },
        dark_mode: !is_latte,
        ..old
    };
}

fn catppuccin_widget_visual(
    old: egui::style::WidgetVisuals,
    theme: CatppuccinTheme,
    bg_fill: egui::Color32,
) -> egui::style::WidgetVisuals {
    egui::style::WidgetVisuals {
        bg_fill,
        weak_bg_fill: bg_fill,
        bg_stroke: egui::Stroke {
            color: theme.overlay1,
            ..old.bg_stroke
        },
        fg_stroke: egui::Stroke {
            color: theme.text,
            ..old.fg_stroke
        },
        ..old
    }
}
