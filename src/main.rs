use crate::app::App;
use eframe::egui;

mod app;
mod components;
mod views;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_title("Lemon-MBL Dev Tool"),
        ..Default::default()
    };

    eframe::run_native(
        "Lemon-MBL Dev Tool",
        options,
        Box::new(|_cc| Ok(Box::new(App::default()))),
    ).unwrap();
}
