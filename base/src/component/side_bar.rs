use crate::TemplateApp;
use egui::{InnerResponse, SidePanel, Ui};

pub fn default(app: &mut TemplateApp, ui: &mut Ui) -> Option<InnerResponse<()>> {
    SidePanel::left("side_bar")
        .resizable(false)
        .show_animated_inside(ui, app.side_bar_show, |ui| {
            ui.heading("eframe template2");
        })
}

pub fn change_side_bar_show(app: &mut TemplateApp) {
    app.side_bar_show = !app.side_bar_show;
}
