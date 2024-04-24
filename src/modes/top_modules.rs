use egui::Ui;

use crate::App;

use super::Mode;

pub(crate) enum TopModule {
    ChangeTitle(usize),
    Menu,
    ShowTitle(usize),
}

pub fn change_title(app: &mut App, ui: &mut Ui, index: usize) {
    ui.vertical_centered(|ui| {
        ui.heading("Title");
        ui.add(egui::TextEdit::singleline(
            &mut app.file_controller.quizes[index].title,
        ));
        ui.add_space(5.);
    });
}

pub fn menu(app: &mut App, ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Quiz App");
        ui.add_space(5.);
        if ui.add(egui::Button::new("Create")).clicked() {
            Mode::change_mode("create", app, app.file_controller.quizes.len() - 1, false);
        }
        ui.add_space(5.);
    });
}

pub fn show_title(app: &mut App, ui: &mut Ui, index: usize) {
    ui.vertical_centered(|ui| {
        ui.heading(&app.file_controller.quizes[index].title);
    });
}
