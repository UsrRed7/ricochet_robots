mod game;

use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();

    let _ = eframe::run_native("RicochetRobots", native_options, Box::new(|cc| Box::new(GUI::new(cc))));
}

#[derive(Default)]
struct GUI {
    buttonstate: bool,
}

impl GUI {
    #[allow(unused_variables)]
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
    #[allow(unused_variables)]
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello world!");
        });
        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("Side panel...");
            ui.horizontal(|ui| {
                ui.label(self.buttonstate.to_string());
                if ui.button("Play").clicked() {
                    self.buttonstate = !self.buttonstate;
                }
            });
        });
        egui::TopBottomPanel::bottom("menu").show(ctx, |ui| {
            ui.heading("Menu!!");
        });
    }
}