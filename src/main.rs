mod config;
mod translations;
mod app;
mod commands;

use app::ControlPanelApp;
use eframe::egui::{self, FontData, FontDefinitions};
use std::env;

fn main() -> Result<(), eframe::Error> {
    env::set_var("WINIT_UNIX_BACKEND", "x11");

    let app = ControlPanelApp::new();
    let window_size = egui::vec2(app.window_width, app.window_height);

    let mut fonts = FontDefinitions::default();
    let font_data: &'static [u8] = include_bytes!("./STXIHEI.TTF");
    fonts.font_data.insert(
        "stxihei".to_owned(),
        FontData::from_static(font_data),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "stxihei".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "stxihei".to_owned());

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(window_size),
        ..Default::default()
    };

    let title = app.tr("window_title");
    eframe::run_native(
        &title,
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_fonts(fonts);
            cc.egui_ctx.set_style(egui::Style {
                visuals: if app.dark_mode { egui::Visuals::dark() } else { egui::Visuals::light() },
                ..Default::default()
            });
            Box::new(app)
        }),
    )
}