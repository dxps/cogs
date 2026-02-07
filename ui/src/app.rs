use crate::{
    UiState,
    comps::{AppComponent, Footer, Header},
    constants::APP_KEY,
    handle_msg,
    messages::UiMessage,
    views::{AppView, Explore, ExploreCategory, ExploreKind, Home, Login, Settings, ViewName},
};
use cogs_shared::domain::model::meta::Kind;
use egui::{
    FontData,
    epaint::text::{FontInsert, InsertFontFamily},
};
use std::sync::mpsc::{Receiver, Sender, channel};

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

        // Load previous app state (if any).
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //

        // State related logistics.
        if self.state.explore.category != ExploreCategory::Templates {
            self.state.explore.kind = ExploreKind::All;
        }

        // Note: Put your widgets into one of the following containers:
        // `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        Header::show(self, ctx);

        if let Ok(res) = self.recvr.try_recv() {
            log::info!("Received msg {:?}", res);
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
                    self.state.data.fetch_all_attr_templates(ctx, self.sendr.clone());
                    self.state.data.fetch_done = true;
                    ctx.request_repaint();
                }

                UiMessage::AttrTemplateDeleted(_) => {
                    self.state.data.fetch_all_attr_templates(ctx, self.sendr.clone());
                    self.state.data.fetch_done = true;
                    ctx.request_repaint();
                }

                UiMessage::ElementCreated(kind, ars) => {
                    match ars {
                        Ok(id) => match kind {
                            Kind::Item => todo!(),
                            Kind::ItemTemplate => {
                                self.state.explore.open_windows_item_template.remove(&id);
                                self.state.data.fetch_all_item_templates(ctx, self.sendr.clone());
                                self.state.data.fetch_done = true;
                                ctx.request_repaint();
                            }
                            Kind::AttributeTemplate => {
                                self.state.data.fetch_all_attr_templates(ctx, self.sendr.clone());
                                self.state.data.fetch_done = true;
                                ctx.request_repaint();
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
                            self.state.data.fetch_all_attr_templates(ctx, self.sendr.clone());
                            self.state.data.fetch_done = true;
                            ctx.request_repaint();
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
                            self.state.data.fetch_all_item_templates(ctx, self.sendr.clone());
                            self.state.data.fetch_done = true;
                            ctx.request_repaint();
                        }
                        Kind::AttributeTemplate => {
                            self.state.data.fetch_all_attr_templates(ctx, self.sendr.clone());
                            self.state.data.fetch_done = true;
                            ctx.request_repaint();
                        }
                        Kind::LinkTemplate => todo!(),
                    },
                    Err(_) => todo!(),
                },

                UiMessage::AttrTemplatesFetched(data) => {
                    self.state.data.fetch_done = true;
                    match data {
                        Ok(attr_templates) => {
                            self.state.data.set_attr_templates(attr_templates);
                        }
                        Err(err) => {
                            log::error!("[app.update] Error fetching attr templates: {}", err);
                        }
                    }
                }

                UiMessage::ItemTemplatesFetched(data) => {
                    self.state.data.fetch_done = true;
                    match data {
                        Ok(item_templates) => {
                            self.state.data.set_item_templates(item_templates);
                        }
                        Err(err) => {
                            log::error!("[app.update] Error fetching item templates: {}", err);
                        }
                    }
                }
            }
        }

        match self.state.curr_view() {
            ViewName::Home => Home::show(self, ctx),
            ViewName::Explore => Explore::show(self, ctx),
            ViewName::Settings => Settings::show(self, ctx),
            ViewName::Login => {
                self.state.set_curr_view(ViewName::Login);
                Login::show(self, ctx);
            }
        }

        egui::TopBottomPanel::bottom("footer_panel")
            .show_separator_line(false)
            .show(ctx, |ui| {
                Footer::show(self, ui);
            });
    }
}

fn ui_init_cosmetics(ctx: &egui::Context) {
    // Set the theme.
    match ctx.theme() {
        egui::Theme::Light => {
            catppuccin_egui::set_theme(ctx, catppuccin_egui::LATTE);
        }
        egui::Theme::Dark => {
            catppuccin_egui::set_theme(ctx, catppuccin_egui::FRAPPE);
        }
    }
}
