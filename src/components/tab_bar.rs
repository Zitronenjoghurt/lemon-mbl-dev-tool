use eframe::egui;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum MenuItem {
    Home = 0,
    Example = 1,
}

pub struct TabBar {
    selected_item: MenuItem,
}

impl Default for TabBar {
    fn default() -> Self {
        Self {
            selected_item: MenuItem::Home,
        }
    }
}

impl TabBar {
    pub fn show(&mut self, ui: &mut egui::Ui) -> MenuItem {
        egui::TopBottomPanel::top("top_bar")
            .frame(egui::Frame::none().inner_margin(4.0))
            .show_inside(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    if ui.selectable_label(self.selected_item == MenuItem::Home, "🏠 Home").clicked() {
                        self.selected_item = MenuItem::Home;
                    }
                    if ui.selectable_label(self.selected_item == MenuItem::Example, "📝 Example").clicked() {
                        self.selected_item = MenuItem::Example;
                    }
                });
            });
        self.selected_item
    }
}