use crate::TemplateApp;
use egui::{Frame, Image, Ui, Widget};

pub(crate) fn default(app: &mut TemplateApp, ui: &mut Ui) {
    egui::SidePanel::left("channel_bar")
        .resizable(false)
        .show_inside(ui, |ui| {
            egui::CentralPanel::default().show_inside(ui, |ui| {});
            ui.horizontal(|ui| {
                ui.label("channel name");
            });
        });
}
