use egui::FontFamily::Proportional;
use egui::FontId;
use egui::Stroke;
use egui::TextStyle::*;
use egui::style::WidgetVisuals;
use egui::style::Widgets;

pub const PADDING: f32 = 5.;
pub const BLACKISH: egui::Color32 = egui::Color32::from_rgb(60, 60, 60);
pub const LESS_BLACK: egui::Color32 = egui::Color32::from_rgb(50, 50, 50);
pub const WHITE: egui::Color32 = egui::Color32::from_rgb(240, 240, 240);

pub fn set_visuals(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    style.text_styles = [
        (Heading, FontId::new(30., Proportional)),
        (Body, FontId::new(18., Proportional)),
        (Monospace, FontId::new(14., Proportional)),
        (Button, FontId::new(18., Proportional)),
        (Small, FontId::new(10., Proportional)),
    ]
    .into();

    style.visuals.dark_mode = true;
    style.visuals.override_text_color = Some(WHITE);

    style.visuals.widgets = Widgets {
        hovered: WidgetVisuals {
            bg_fill: BLACKISH,
            weak_bg_fill: BLACKISH,
            bg_stroke: Stroke::new(1., WHITE),
            rounding: egui::Rounding::same(5.),
            fg_stroke: Stroke::new(1., WHITE),
            expansion: 3.,
        },
        inactive: WidgetVisuals {
            bg_fill: LESS_BLACK,
            weak_bg_fill: LESS_BLACK,
            bg_stroke: Stroke::NONE,
            rounding: egui::Rounding::same(5.),
            fg_stroke: Stroke::NONE,
            expansion: 1.0,
        },
        open: WidgetVisuals {
            bg_fill: LESS_BLACK,
            weak_bg_fill: LESS_BLACK,
            bg_stroke: Stroke::NONE,
            rounding: egui::Rounding::same(5.),
            fg_stroke: Stroke::NONE,
            expansion: 1.0,
        },
        ..Default::default()
    };

    ctx.set_style(style);
}
