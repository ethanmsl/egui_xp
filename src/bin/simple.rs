//! Just playin'
//!
//!

use eframe::egui;
use egui::{Pos2, pos2};
use egui_xp::Result as MyResult; // NOTE: this is mine.
use egui_xp::active_global_default_tracing_subscriber;

fn main() -> MyResult<()> {
        let _log_writer_guard = active_global_default_tracing_subscriber()?;

        let native_options = eframe::NativeOptions::default();
        eframe::run_native("AppName", native_options, Box::new(|cc| Ok(Box::new(AppStruct::new(cc)))))?;
        Ok(())
}

impl eframe::App for AppStruct {
        fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
                ctx.set_pixels_per_point(2.0);
                egui::Window::new("My Window")
                        .default_open(true)
                        .open(&mut self.open_var)
                        // .constrain(false)
                        .constrain_to(egui::Rect { min: pos2(10., 10.), max: pos2(1000., 200.) })
                        .title_bar(false)
                        .interactable(self.interactable) // button works either way
                        .enabled(self.enabled)
                        .show(ctx, |ui| {
                                ui.heading("this is a ui.heading.");
                                ui.label("this is a ui.label.");

                                if ui.button("Boop print").clicked() {
                                        println!("boop");
                                };
                        });

                egui::CentralPanel::default().show(ctx, |ui| {
                        ui.heading("this is a ui.heading.");
                        ui.label("this is a ui.label.");

                        if ui.button("toggle open_var").clicked() {
                                self.open_var = !self.open_var;
                        };
                        if ui.button("toggle interactable").clicked() {
                                self.interactable = !self.interactable;
                        };
                        ui.label(format!("open_var is: {}", self.open_var));
                        ui.label(format!("interactable is: {}", self.interactable));
                        if ui.button("toggle enabledness").clicked() {
                                self.enabled = !self.enabled;
                                ui.label(format!("enabled is: {}", self.enabled));
                        };
                        ui.label(format!("enabled is: {}", self.enabled));
                });
        }
}

#[derive(Default)]
struct AppStruct {
        // Optionally add fields and data here
        open_var:     bool,
        interactable: bool,
        enabled:      bool,
}

impl AppStruct {
        fn new(_cc: &eframe::CreationContext<'_>) -> Self {
                // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
                // Restore app state using cc.storage (requires the "persistence" feature).
                // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
                // for e.g. egui::PaintCallback.
                Self::default()
        }
}
