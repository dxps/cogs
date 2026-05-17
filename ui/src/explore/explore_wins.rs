use egui::Ui;

use crate::{
    CogsApp,
    comps::{AppComponent, AttrTemplateWindow, ItemTemplateWindow, ItemWindow},
    constants::EXPLORE_ELEMENT,
    security::AccessLevelWindow,
};

pub fn show_windows(ctx: &mut CogsApp, ui: &mut Ui, ectx: &egui::Context) {
    for (_, element) in ctx.state.explore.open_windows_item_template.clone().iter() {
        ectx.data_mut(|d| d.insert_temp(egui::Id::from(EXPLORE_ELEMENT), element.clone()));
        ItemTemplateWindow::show(ctx, ui);
    }
    for (_, element) in ctx.state.explore.open_windows_attr_template.clone().iter() {
        ectx.data_mut(|d| d.insert_temp(egui::Id::from(EXPLORE_ELEMENT), element.clone()));
        AttrTemplateWindow::show(ctx, ui);
    }
    for (_, element) in ctx.state.explore.open_windows_item.clone().iter() {
        ectx.data_mut(|d| d.insert_temp(egui::Id::from(EXPLORE_ELEMENT), element.clone()));
        ItemWindow::show(ctx, ui);
    }
    for (_, element) in ctx.state.explore.open_windows_access_level.clone().iter() {
        ectx.data_mut(|d| d.insert_temp(egui::Id::from(EXPLORE_ELEMENT), element.clone()));
        AccessLevelWindow::show(ctx, ui);
    }
}
