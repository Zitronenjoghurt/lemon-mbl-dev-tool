use eframe::egui;

pub fn show(ui: &mut egui::Ui) {
    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Example");
            if ui.button("Sample Button").clicked() {
                println!("Example button clicked!");
            }
        });
    });
}