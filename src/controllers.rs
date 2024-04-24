use std::path::Path;

use crate::{
    quiz_structs::{Quiz, Word},
    App,
};

use self::pop_ups::*;
use self::round::*;

pub mod pop_ups;
pub mod round;

pub struct RoundController {
    round: Round,
}

pub struct FileController {
    pub quizes: Vec<Quiz>,
    pub titles: Vec<String>,
    pub temp_title: String,
}

pub struct PopUpController {
    pub pop_up: PopUp,
    pub enabled: bool,
}

impl PopUpController {
    pub fn match_popup(app: &mut App, ctx: &egui::Context) {
        match &app.popup_controller.pop_up {
            PopUp::Nothing => {}
            PopUp::Delete(index) => delete_popup(app, ctx, *index),
            PopUp::Fill(text) => fill_popup(app, ctx, text.clone()),
        };
    }
}

impl FileController {
    pub fn reload_data(&mut self) {
        (self.quizes, self.titles) = reload_quizes();
        self.temp_title = "".to_string();
    }
}

impl Default for RoundController {
    fn default() -> Self {
        Self {
            round: Round { cards: Vec::new() },
        }
    }
}

impl Default for FileController {
    fn default() -> Self {
        let (temp1, temp2) = reload_quizes();

        Self {
            quizes: temp1,
            titles: temp2,
            temp_title: "".to_string(),
        }
    }
}

impl Default for PopUpController {
    fn default() -> Self {
        Self {
            pop_up: PopUp::Nothing,
            enabled: true,
        }
    }
}

fn reload_quizes() -> (Vec<Quiz>, Vec<String>) {
    let mut quizes: Vec<Quiz> = Vec::new();

    let mut titles: Vec<String> = Vec::new();
    let paths = std::fs::read_dir("./quizes/").unwrap();

    // Reading and writing to vec, every title from quizes directory
    for path in paths {
        // I need to later add error Handling
        let test = path.unwrap().file_name().clone();
        let _path = Path::new(&test).file_stem().unwrap().to_str().unwrap();
        titles.push(_path.to_string());
    }

    // Sorting the titles, because they don't come up the same everytime
    // probably due to the way files are stored in linux dictionary
    titles.sort();

    // Loading every quiz
    for title in titles.clone() {
        quizes.push(Quiz::load_data(title));
    }

    // Last quiz would be used as empty to the creation process
    quizes.push(Quiz::new(
        "".to_string(),
        "".to_string(),
        vec![Word::new_empty(), Word::new_empty(), Word::new_empty()],
    ));

    (quizes, titles)
}
