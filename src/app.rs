use egui::{
    Color32, FontData, RichText,
    epaint::text::{FontInsert, InsertFontFamily},
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct CogsApp {
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for CogsApp {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            value: 2.5,
        }
    }
}

impl CogsApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Self::setup_font(&cc.egui_ctx);

        cc.egui_ctx.set_zoom_factor(1.2);

        // Load previous app state (if any).
        // Note: The `persistence` feature must be enabled for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }

    fn setup_font(ctx: &egui::Context) {
        ctx.add_font(FontInsert::new(
            "Supreme",
            FontData::from_static(include_bytes!("../assets/fonts/Supreme-Regular.ttf")),
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
}

impl eframe::App for CogsApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
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

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Cogs")
                .on_hover_cursor(egui::CursorIcon::Help)
                .on_hover_text("Cogs is a cognitive platform for cognitive needs.");

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Enter label:");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));

            if ui.button("Increment").clicked() {
                self.value += 0.5;
            }

            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("Label: {}", self.label)).color(Color32::MAGENTA));
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                copyright_footer(ui);
                // egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn copyright_footer(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
