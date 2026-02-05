#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively.
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    init_logging();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                // Note: Adding an icon is optional.
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-192.png")[..]).expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native("Cogs", native_options, Box::new(|cc| Ok(Box::new(cogs_ui::CogsApp::new(cc)))))
}

// When compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends.
    // eframe::WebLogger::init(log::LevelFilter::Debug).ok();
    init_logging();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window().expect("No window").document().expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|mut cc| {
                    let mut app = cogs_ui::CogsApp::new(cc);
                    app.init_web(&mut cc);
                    Ok(Box::new(app))
                }),
            )
            .await;

        // Remove the loading text and spinner.
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html("<p> The app has crashed. See the developer console for details. </p>");
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}

use std::sync::Once;
static LOGGING_INIT: Once = Once::new();

pub fn init_logging() {
    LOGGING_INIT.call_once(|| {
        let _ = tracing_log::LogTracer::init();

        #[cfg(not(target_arch = "wasm32"))]
        {
            use tracing_subscriber::{EnvFilter, fmt};

            let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,eframe=warn,egui_glow=warn"));

            let _ = fmt().with_env_filter(filter).try_init(); // <- do not panic if already set
        }

        #[cfg(target_arch = "wasm32")]
        {
            use tracing_subscriber::{EnvFilter, fmt, prelude::*};
            use tracing_subscriber_wasm::MakeConsoleWriter;

            let filter = EnvFilter::new("info,eframe=warn,egui_glow=warn");

            let subscriber = tracing_subscriber::registry().with(filter).with(
                fmt::layer()
                    .with_writer(MakeConsoleWriter::default())
                    .without_time()
                    .with_target(false)
                    .with_file(true)
                    .with_line_number(true)
                    .with_ansi(false),
            );

            // non-panicking:
            let _ = tracing::subscriber::set_global_default(subscriber);
        }
    });
}
