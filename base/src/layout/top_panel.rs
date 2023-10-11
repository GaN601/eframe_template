use crate::component;
use crate::TemplateApp;
use egui::{Context, InnerResponse, Layout};

pub(crate) fn default(
    app: &mut TemplateApp,
    ctx: &Context,
    _frame: &mut eframe::Frame,
) -> InnerResponse<()> {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        // The top panel is often a good place for a menu bar:

        egui::menu::bar(ui, |ui| {
            if ui.button("💻 Backend").clicked() {
                component::side_bar::change_side_bar_show(app);
            }
            #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
            {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
                ui.add_space(16.0);
            }

            ui.label("Welcome");
            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
    })
}
