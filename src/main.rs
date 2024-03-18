mod game;

use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();

    let _ = eframe::run_native("RicochetRobots", native_options, Box::new(|cc| Box::new(GUI::new(cc))));
}

#[derive(Default)]
struct GUI {}

impl GUI {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        
        // Perhaps also have font/theme options saved and set with ctx

        // TODO: find example of and implement persistence

        Self::default()
    }
}

impl eframe::App for GUI {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Hello World!");
       });
   }
}