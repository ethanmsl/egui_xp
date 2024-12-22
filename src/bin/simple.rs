//! Just playin'
//!
//!

use eframe::egui;
use egui_xp::Result as MyResult; // NOTE: this is mine.
use egui_xp::active_global_default_tracing_subscriber;

fn main() -> MyResult<()> {
        println!("Hello, world!");
        let _log_writer_guard = active_global_default_tracing_subscriber()?;

        let native_options = eframe::NativeOptions::default();
        eframe::run_native("AppName", native_options, Box::new(|cc| Ok(Box::new(AppStruct::new(cc)))))?;
        Ok(())
}

#[derive(Default)]
struct AppStruct {
        // Optionally add fields and data here
}

impl AppStruct {
        fn new(cc: &eframe::CreationContext<'_>) -> Self {
                // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
                // Restore app state using cc.storage (requires the "persistence" feature).
                // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
                // for e.g. egui::PaintCallback.
                Self::default()
        }
}

impl eframe::App for AppStruct {
        fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
                egui::CentralPanel::default().show(ctx, |ui| {
                        ui.heading("Hello, world!");
                        ui.label("This is an Egui app.");
                        ui.label("This is a template app.");
                });
        }
}
