use std::sync::{Arc, RwLock};

use crate::{
    constants::APP_KEY,
    header_footer::{UserWidget, footer},
    view::ViewType,
};
use cogs_shared::{app::AppError, domain::model::UserAccount};
use egui::{
    FontData,
    epaint::text::{FontInsert, InsertFontFamily},
};

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AppState {
    pub view_type: ViewType,
}

#[derive(serde::Deserialize, serde::Serialize)] // so we can persist ui state on app shutdown.
#[serde(default)] // if we add new fields, give them default values when deserializing old state.
pub struct CogsApp {
    pub(crate) label: String,

    #[serde(skip)] // don't serialize this field.
    pub(crate) value: f32,

    pub(crate) state: Arc<RwLock<AppState>>,

    pub(crate) auth_session: Option<UserAccount>,
    pub(crate) auth_error: Option<AppError>,
    #[serde(skip)]
    pub(crate) user_widget: UserWidget,
}

impl Default for CogsApp {
    fn default() -> Self {
        let state = Arc::new(RwLock::new(AppState::default()));
        Self {
            label: "Hello World!".to_owned(),
            value: 2.5,
            state: state.clone(),
            auth_session: None,
            auth_error: None,
            user_widget: UserWidget::new(state),
        }
    }
}

impl CogsApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Self::init_font(&cc.egui_ctx);

        // Image loading init.
        egui_extras::install_image_loaders(&cc.egui_ctx);

        cc.egui_ctx.set_zoom_factor(1.2);

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
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //
        match ctx.theme() {
            egui::Theme::Light => {
                catppuccin_egui::set_theme(ctx, catppuccin_egui::LATTE);
            }
            egui::Theme::Dark => {
                catppuccin_egui::set_theme(ctx, catppuccin_egui::FRAPPE);
            }
        }

        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        self.top_header(ctx);

        let state = self.state.clone();
        log::info!("view_type: {:#?}", state.read().unwrap().view_type);
        match state.read().unwrap().view_type {
            ViewType::Home => self.home(ctx),
            ViewType::Explore => self.home(ctx),
            ViewType::Settings => self.home(ctx),
            ViewType::Login => self.login(ctx),
        }

        // ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        // footer(ui);
        // egui::warn_if_debug_build(ui);
        // });
        egui::TopBottomPanel::bottom("footer_panel")
            .show_separator_line(false)
            .show(ctx, |ui| {
                footer(ui);
            });
    }
}
