use egui::epaint;
use egui::Style;
use egui::Ui;

use crate::{
    quiz_structs::{Quiz, Word},
    App,
};

use self::{bottom_modules::*, central_modules::*, top_modules::*};

mod bottom_modules;
mod central_modules;
mod top_modules;

pub struct Mode {
    pub(crate) top_module: TopModule,
    pub(crate) central_module: CentralModule,
    pub(crate) bottom_module: BottomModule,
    pub top_frame: egui::Frame,
    pub central_frame: egui::Frame,
    pub bottom_frame: egui::Frame,
}

impl Mode {
    pub fn match_top(app: &mut App, ui: &mut Ui) {
        match app.mode.top_module {
            TopModule::ChangeTitle(index) => change_title(app, ui, index),
            TopModule::Menu => menu(app, ui),
            TopModule::ShowTitle(index) => show_title(app, ui, index),
        }
    }

    pub fn match_central(app: &mut App, ui: &mut Ui) {
        match app.mode.central_module {
            CentralModule::List => list(app, ui),
            CentralModule::Create(index) => creation(app, ui, index),
            CentralModule::Quiz(index) => quiz(app, ui, index),
        }
    }

    pub fn match_bottom(app: &mut App, ui: &mut Ui) {
        match app.mode.bottom_module {
            BottomModule::Nothing => {}
            BottomModule::BackSave(index, is_edited) => back_save(app, ui, index, is_edited),
            BottomModule::BackNext(index) => back_next(app, ui, index),
        }
    }

    pub fn change_mode(change: &str, app: &mut App, index: usize, is_edited: bool) {
        match change.to_lowercase().as_str() {
            "list" => {
                let temp = app.file_controller.quizes.len() - 1;

                app.mode.top_module = TopModule::Menu;
                app.mode.central_module = CentralModule::List;
                app.mode.bottom_module = BottomModule::Nothing;
                app.file_controller.quizes[temp] = Quiz::new(
                    "".to_string(),
                    "".to_string(),
                    vec![Word::new_empty(), Word::new_empty(), Word::new_empty()],
                )
            }
            "create" => {
                app.mode.top_module = TopModule::ChangeTitle(index);
                app.mode.central_module = CentralModule::Create(index);
                app.mode.bottom_module = BottomModule::BackSave(index, is_edited);
            }
            "play" => {
                app.mode.top_module = TopModule::ShowTitle(index);
                app.mode.central_module = CentralModule::Quiz(index);
                app.mode.bottom_module = BottomModule::BackNext(index);
            }
            _ => {}
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        let rounding: egui::Rounding = egui::Rounding::same(5.);
        let fill: egui::Color32 = egui::Color32::from_rgb(27, 27, 27);
        let stroke: egui::Stroke = egui::Stroke::NONE;
        let frame: egui::Frame = egui::Frame {
            inner_margin: egui::Margin::symmetric(10., 10.),
            outer_margin: egui::Margin::same(0.),
            rounding,
            shadow: epaint::Shadow::NONE,
            fill,
            stroke,
        };

        Self {
            top_module: TopModule::Menu,
            central_module: CentralModule::List,
            bottom_module: BottomModule::Nothing,
            top_frame: frame,
            central_frame: frame,
            bottom_frame: frame,
        }
    }
}
