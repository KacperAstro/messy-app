use crate::App;

pub enum PopUp {
    Nothing,
    Delete(usize),
    Fill(String),
}

pub fn delete_popup(app: &mut App, ctx: &egui::Context, index: usize) {
    app.popup_controller.enabled = false;
    egui::Window::new("test")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0., -50.))
        .collapsible(false)
        .title_bar(false)
        .resizable(false)
        .frame(
            egui::Frame::window(&ctx.style()).shadow(egui::epaint::Shadow {
                color: egui::Color32::TRANSPARENT,
                ..Default::default()
            }),
        )
        .show(ctx, |ui| {
            ui.label("Do you want to delete this?");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                if ui.add(egui::Button::new("Yes")).clicked() {
                    std::fs::remove_file(format!(
                        "quizes/{}.json",
                        app.file_controller.quizes[index].title
                    ))
                    .unwrap();
                    app.file_controller.quizes.remove(index);
                    app.popup_controller.enabled = true;
                    app.popup_controller.pop_up = PopUp::Nothing;
                    app.file_controller.reload_data();
                }

                if ui.add(egui::Button::new("No")).clicked() {
                    app.popup_controller.enabled = true;
                    app.popup_controller.pop_up = PopUp::Nothing;
                }
            });
        });
}

pub fn fill_popup(app: &mut App, ctx: &egui::Context, text: String) {
    app.popup_controller.enabled = false;
    egui::Window::new("test")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0., -50.))
        .collapsible(false)
        .title_bar(false)
        .resizable(false)
        .frame(
            egui::Frame::window(&ctx.style()).shadow(egui::epaint::Shadow {
                color: egui::Color32::TRANSPARENT,
                ..Default::default()
            }),
        )
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(text);
                if ui.add(egui::Button::new("Okay")).clicked() {
                    app.popup_controller.enabled = true;
                    app.popup_controller.pop_up = PopUp::Nothing;
                }
            });
        });
}
