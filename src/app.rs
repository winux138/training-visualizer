use eframe::egui::{
    self, accesskit::DefaultActionVerb, load::DefaultBytesLoader, DeferredViewportUiCallback,
};

#[derive(Default)]
pub struct App {}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Some extra init
        cc.egui_ctx.set_visuals(egui::Visuals::light());
        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);

                    egui::widgets::global_dark_light_mode_buttons(ui);
                });
            });
        });
    }
}
