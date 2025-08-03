#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::CogsApp;

mod states;
pub use states::*;

mod consts;

mod comps;
mod views;

pub mod messages;
