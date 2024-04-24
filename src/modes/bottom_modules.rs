use egui::Ui;

use crate::{controllers::pop_ups::PopUp, visuals::PADDING, App};

use super::Mode;

pub(crate) enum BottomModule {
    Nothing,
    BackSave(usize, bool),
    BackNext(usize),
}

pub fn back_save(app: &mut App, ui: &mut Ui, index: usize, is_edited: bool) {
    ui.add_space(PADDING);
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            if ui.add(egui::Button::new("Back")).clicked() {
                Mode::change_mode("list", app, index, is_edited);
            }
        });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.add(egui::Button::new("Save")).clicked() {
                if app.file_controller.quizes[index].title != ""
                    && app.file_controller.quizes[index].desc != ""
                    && app.file_controller.quizes[index]
                        .quiz_data
                        .original
                        .clone()
                        .into_iter()
                        .any(|x| x.original != "" && x.translation != "")
                {
                    if is_edited == false {
                        if !app
                            .file_controller
                            .titles
                            .contains(&app.file_controller.quizes[index].title)
                        {
                            // deleting empty entries
                            app.file_controller.quizes[index]
                                .quiz_data
                                .original
                                .retain(|x| x.original != "" && x.translation != "");

                            app.file_controller.quizes[index].save_data();

                            app.file_controller.reload_data();
                            Mode::change_mode("list", app, index, is_edited);
                        } else {
                            app.popup_controller.pop_up = PopUp::Fill(
                                "Change the Title because there is already a Quiz with this Title"
                                    .to_string(),
                            );
                        }
                    } else {
                        let temp_quiz = app.file_controller.quizes[index].clone();
                        app.file_controller
                            .titles
                            .retain(|x| x != &app.file_controller.temp_title);

                        if !app
                            .file_controller
                            .titles
                            .contains(&app.file_controller.quizes[index].title)
                        {
                            std::fs::remove_file(format!(
                                "quizes/{}.json",
                                app.file_controller.temp_title
                            ))
                            .unwrap();
                            temp_quiz.save_data();
                            app.file_controller.reload_data();
                            Mode::change_mode("list", app, index, is_edited);
                        } else {
                            app.popup_controller.pop_up = PopUp::Fill(
                                "Change the Title because there is already a Quiz with this Title"
                                    .to_string(),
                            );
                        }
                    }
                } else {
                    app.popup_controller.pop_up = PopUp::Fill(
                        "You have to fill Title, Descriptions and have atleat one word filled."
                            .to_string(),
                    );
                }
            }
        });
    });
}

pub fn back_next(app: &mut App, ui: &mut Ui, index: usize) {
    ui.add_space(PADDING);
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            if ui.add(egui::Button::new("Back")).clicked() {
                Mode::change_mode("list", app, index, false);
            }
        });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.add(egui::Button::new("Next")).clicked() {}
        });
    });
}
