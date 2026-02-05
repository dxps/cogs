#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::CogsApp;

mod app_msg_handlers;
pub(self) use app_msg_handlers::*;

mod state;
pub use state::*;

mod constants;

mod comps;
mod views;

pub mod messages;
