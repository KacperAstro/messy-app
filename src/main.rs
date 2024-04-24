use controllers::*;
use eframe::egui;
use modes::Mode;
use visuals::set_visuals;

mod controllers;
mod modes;
mod quiz_structs;
mod visuals;

// TODO
//
// "The Round System"
//---------------------------------------------------------------------------------------------
// every round consists of 7 words that are taken from "original", "multi_choice", "f_to_s"
// If user has guessed correctly, then word is moved to the next vec.
//---------------------------------------------------------------------------------------------
// At the start algorithms, sees which vec has the biggest amount of elements, then picks one
// from the one that has the most and it does this until round_vec does have 7 elements.
//---------------------------------------------------------------------------------------------
// How to show it?
// There would be a match statement which would check what type of the card it is and based on
// that show the according card.
//
//"Saving System"
// First check when user clicks "Next" button if quiz_data.original.len > 0 then we can procide to
// the next tab "Title & Desc" if not user should be prompted that quiz is empty. In this Tab when
// user clicks "Save Button" we check if quiz with the same name exists in quizes dictionary if not
// save otherwise ask user if he wants to overwrite the quiz.
// --------------------------------------------------------------------------------------------------
// What to do when user is editing the quiz? Should user be prompted or not ?
//
//"Fixes"
// Fix the problem with edit state
// Remove every empty word upon saving data
//
// "Module System"
// There would be three modules. "Top", "Central" and "Bottom" module

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640., 720.)), // Good size 640, 720
        resizable: false,
        centered: true,
        ..Default::default()
    };

    eframe::run_native("Quiz App", options, Box::new(|cc| Box::new(App::new(cc))))
}
pub struct App {
    mode: Mode,
    file_controller: FileController,
    popup_controller: PopUpController,
    round_controller: RoundController,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        set_visuals(&cc.egui_ctx);

        Self {
            mode: Mode::default(),
            file_controller: FileController::default(),
            popup_controller: PopUpController::default(),
            round_controller: RoundController::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("Top Module")
            .frame(self.mode.top_frame)
            .show(ctx, |ui| {
                ui.add_enabled_ui(self.popup_controller.enabled, |ui| {
                    Mode::match_top(self, ui);
                });
            });

        egui::TopBottomPanel::bottom("Bottom Module")
            .frame(self.mode.bottom_frame)
            .show(ctx, |ui| {
                ui.add_enabled_ui(self.popup_controller.enabled, |ui| {
                    Mode::match_bottom(self, ui);
                });
            });

        PopUpController::match_popup(self, ctx);

        egui::CentralPanel::default()
            .frame(self.mode.central_frame)
            .show(ctx, |ui| {
                ui.add_enabled_ui(self.popup_controller.enabled, |ui| {
                    Mode::match_central(self, ui);
                });
            });
    }
}
