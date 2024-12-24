use eframe::egui;

pub fn show(ui: &mut egui::Ui) {
    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Home");
            ui.label("Welcome to the Lemon-MBL Dev Tool!");
            if ui.button("Sample Button").clicked() {
                println!("Home button clicked!");
            }
        });
    });
}