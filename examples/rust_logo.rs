fn main() {
    eframe::run_native(
        "egui_nerdfonts demo",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(Demo::new(cc)))),
    )
    .unwrap();
}

struct Demo {}

impl Demo {
    fn new(cc: &eframe::CreationContext) -> Self {
        let mut fonts = egui::FontDefinitions::default();

        egui_nerdfonts::add_to_fonts(&mut fonts, egui_nerdfonts::Variant::Regular);

        cc.egui_ctx.set_fonts(fonts);

        Self {}
    }
}

impl eframe::App for Demo {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx();

        #[allow(deprecated)]
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                ui.label(
                    egui::RichText::new(format!(
                        "egui_nerdfonts::regular::LANGUAGE_RUST: {}",
                        egui_nerdfonts::regular::LANGUAGE_RUST
                    ))
                    .size(42.),
                );
            });
        });
    }
}
