use egui::style::Spacing;
use egui::{Frame, Margin, Ui};

use crate::controllers::pop_ups::PopUp;
use crate::{quiz_structs::Word, visuals::PADDING, App};

use super::Mode;

pub(crate) enum CentralModule {
    List,
    Create(usize),
    Quiz(usize),
}

pub fn creation(app: &mut App, ui: &mut Ui, index: usize) {
    egui::CollapsingHeader::new("Description")
        .default_open(true)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Description: ");
                ui.add(
                    egui::TextEdit::multiline(&mut app.file_controller.quizes[index].desc)
                        .desired_rows(2)
                        .desired_width(480.),
                );
            });
        });
    ui.add_space(5.);
    ui.separator();
    ui.vertical_centered(|ui| {
        egui::ScrollArea::vertical()
            .stick_to_bottom(true)
            .show(ui, |ui| {
                let mut rem_quizes_index: Vec<usize> = Vec::new();

                for i in 0..app.file_controller.quizes[index].quiz_data.original.len() {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        if ui
                            .add(egui::Button::new("-").min_size(egui::vec2(30., 10.)))
                            .clicked()
                        {
                            rem_quizes_index.push(i);
                        }
                    });

                    ui.add_space(5.);

                    ui.horizontal(|ui| {
                        ui.label("Original:       ");
                        ui.add(
                            egui::TextEdit::singleline(
                                &mut app.file_controller.quizes[index].quiz_data.original[i]
                                    .original,
                            )
                            .desired_width(500.),
                        );
                    });

                    ui.add_space(5.);

                    ui.horizontal(|ui| {
                        ui.label("Translation: ");
                        ui.add(
                            egui::TextEdit::singleline(
                                &mut app.file_controller.quizes[index].quiz_data.original[i]
                                    .translation,
                            )
                            .desired_width(500.),
                        );
                    });
                    ui.separator();
                }
                ui.add_space(5.);
                if ui
                    .add(egui::Button::new("+").min_size(egui::vec2(50., 10.)))
                    .clicked()
                {
                    app.file_controller.quizes[index]
                        .quiz_data
                        .original
                        .push(Word::new_empty());
                }

                for x in rem_quizes_index {
                    app.file_controller.quizes[index]
                        .quiz_data
                        .original
                        .remove(x);
                }
            });
    });
}

pub fn list(app: &mut App, ui: &mut Ui) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        for i in 0..(app.file_controller.quizes.len() - 1) {
            ui.add_space(PADDING);
            ui.label(format!("Title: {}", &app.file_controller.quizes[i].title));

            ui.add_space(PADDING);
            ui.label(format!(
                "Description: {}",
                &app.file_controller.quizes[i].desc
            ));

            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                // Due to Layout this is second button
                if ui.button("Play").clicked() {
                    Mode::change_mode("play", app, i, false);
                }

                if ui.button("Edit").clicked() {
                    app.file_controller.temp_title = app.file_controller.quizes[i].title.clone();
                    Mode::change_mode("create", app, i, true);
                }

                if ui.button("Delete").clicked() {
                    app.popup_controller.pop_up = PopUp::Delete(i);
                }
            });

            ui.separator();
        }
    });
}

pub fn quiz(app: &mut App, ui: &mut Ui, index: usize) {
    app.mode.central_frame.inner_margin = Margin::symmetric(20., 20.);
    ui.label("test");
    ui.label("test");
}
