use crate::{component, TemplateApp};
use egui::{Context, Image, InnerResponse, Rangef, SidePanel, Vec2};

fn gen_image<'a>() -> Image<'a> {
    Image::new("file://assets/icon-256.png")
        .fit_to_exact_size(Vec2::new(42.0, 42.0))
        .rounding(10.0)
}
pub(crate) fn default(app: &mut TemplateApp, ctx: &Context) -> InnerResponse<()> {
    egui::SidePanel::left("left_panel_1")
        .resizable(false)
        .width_range(Rangef::new(20.0, 50.0))
        .show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.add(gen_image()).on_hover_text("item 1");
                    ui.add(gen_image()).on_hover_text("item 2");
                    ui.add(gen_image()).on_hover_text("item 3");
                    ui.add(gen_image()).on_hover_text("item 4");
                    ui.add(gen_image()).on_hover_text("item 5");
                    ui.add(gen_image()).on_hover_text("item 6");
                    ui.add(gen_image()).on_hover_text("item 7");
                    ui.add(gen_image()).on_hover_text("item 8");
                    ui.add(gen_image()).on_hover_text("item 9");
                    ui.add(gen_image()).on_hover_text("item 10");
                    ui.add(gen_image()).on_hover_text("item 11");
                    ui.add(gen_image()).on_hover_text("item 12");
                    ui.add(gen_image()).on_hover_text("item 13");
                    ui.add(gen_image()).on_hover_text("item 14");
                    ui.add(gen_image()).on_hover_text("item 15");
                    ui.add(gen_image()).on_hover_text("item 16");
                    ui.add(gen_image()).on_hover_text("item 17");
                    ui.add(gen_image()).on_hover_text("item 18");
                    ui.add(gen_image()).on_hover_text("item 19");
                });
        })
}

pub(crate) fn channel_panel(app: &mut TemplateApp, ctx: &Context) -> InnerResponse<()> {
    SidePanel::left("channel_panel")
        .resizable(false)
        .show(ctx, |ui| {
            component::channel_bar::default(app, ui);
        })
}
