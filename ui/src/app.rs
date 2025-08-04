use std::sync::mpsc::{Receiver, Sender, channel};

use crate::{
    AppState,
    comps::{AppComponent, Footer, Header},
    consts::APP_KEY,
    messages::UiMessage,
    views::{AppView, Explore, ExploreCategory, ExploreKind, Home, Login, Settings, ViewType},
};
use cogs_shared::domain::model::UserAccount;
use egui::{
    FontData,
    epaint::text::{FontInsert, InsertFontFamily},
};

#[derive(serde::Deserialize, serde::Serialize)] // so we can persist ui state on app shutdown.
#[serde(default)] // if we add new fields, give them default values when deserializing old state.
pub struct CogsApp {
    pub(crate) state: AppState,
    pub(crate) auth_session: Option<UserAccount>,

    #[serde(skip)]
    /// Sender for UI messages.
    pub sendr: Sender<UiMessage>,

    #[serde(skip)]
    /// Receiver for UI messages.
    pub recvr: Receiver<UiMessage>,
}

impl Default for CogsApp {
    fn default() -> Self {
        let (sendr, recvr) = channel();
        Self {
            state: AppState::default(),
            auth_session: None,
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
        Self::init_font(&ectx);

        // Image loading init.
        egui_extras::install_image_loaders(ectx);

        // Zoom setting.
        ectx.set_zoom_factor(1.2);

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
    pub fn init_web(&self, cc: &eframe::CreationContext<'_>) {
        log::info!("[init web] {:#?}", cc.integration_info.web_info);
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

        log::info!("[app.update]");
        // State related logistics.
        if self.state.explore.category != ExploreCategory::Templates {
            self.state.explore.kind = ExploreKind::All;
        }

        // Visually, put your widgets into one of the following containers:
        // `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        Header::show(self, ctx);

        if let Ok(res) = self.recvr.try_recv() {
            log::info!("[app.update] Received msg {:?}", res);
            match res {
                UiMessage::Login(account) => match account {
                    Ok(account) => match account {
                        Some(account) => {
                            self.state.auth.user_account = Some(account);
                            self.state.view_type = ViewType::Home;
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
                    self.state.auth.user_account = None;
                    self.state.view_type = ViewType::Home;
                }
                UiMessage::Settings => {
                    self.state.view_type = ViewType::Settings;
                }
            }
        }

        match self.state.view_type {
            ViewType::Home => Home::show(self, ctx),
            ViewType::Explore => Explore::show(self, ctx),
            ViewType::Settings => Settings::show(self, ctx),
            ViewType::Login => Login::show(self, ctx),
        }

        egui::TopBottomPanel::bottom("footer_panel")
            .show_separator_line(false)
            .show(ctx, |ui| {
                Footer::show(self, ui);
            });
    }
}

fn ui_init_cosmetics(ctx: &egui::Context) {
    match ctx.theme() {
        egui::Theme::Light => {
            catppuccin_egui::set_theme(ctx, catppuccin_egui::LATTE);
        }
        egui::Theme::Dark => {
            catppuccin_egui::set_theme(ctx, catppuccin_egui::FRAPPE);
        }
    }
    // let vs = Visuals::dark();
    // ctx.set_visuals(vs);
}
