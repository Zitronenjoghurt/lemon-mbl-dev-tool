use crate::components::tab_bar::{MenuItem, TabBar};
use crate::views;
use eframe::egui::Context;
use eframe::{egui, Frame};
use lemon_mbl::get_game_data;
use lemon_mbl::states::game_data::GameData;

pub struct App {
    game_data: GameData,
    tab_bar: TabBar,
}

impl Default for App {
    fn default() -> Self {
        Self {
            game_data: (*get_game_data()).clone(),
            tab_bar: TabBar::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let selected_item = self.tab_bar.show(ui);

            match selected_item {
                MenuItem::Home => views::home::show(ui),
                MenuItem::Example => views::example::show(ui),
            }
        });
    }
}