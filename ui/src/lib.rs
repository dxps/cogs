#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::CogsApp;

mod ui_state;
pub use ui_state::*;

mod constants;

mod comps;
mod views;

pub mod messages;
