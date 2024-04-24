use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct Word {
    pub original: String,
    pub translation: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct QuizData {
    pub original: Vec<Word>,
    pub multi_choice: Vec<Word>,
    pub f_to_s: Vec<Word>, // First to second
    pub s_to_f: Vec<Word>, // Second to first
}

#[derive(Clone, PartialEq)]
pub struct Quiz {
    pub title: String,
    pub desc: String,
    pub quiz_data: QuizData,
}

#[derive(Deserialize, Serialize)]
struct QuizJson {
    desc: String,
    quiz_data: QuizData,
}

impl Word {
    pub fn new(original: String, translation: String) -> Word {
        Word {
            original,
            translation,
        }
    }

    pub fn new_empty() -> Word {
        Word {
            original: "".to_string(),
            translation: "".to_string(),
        }
    }

    pub fn is_empty(&self) -> bool {
        if self.original == "" || self.translation == "" {
            true
        } else {
            false
        }
    }
}

impl QuizData {
    pub fn new_from_original(original: Vec<Word>) -> QuizData {
        QuizData {
            original,
            multi_choice: vec![],
            f_to_s: vec![],
            s_to_f: vec![],
        }
    }

    fn new_empty() -> QuizData {
        QuizData {
            original: vec![],
            multi_choice: vec![],
            f_to_s: vec![],
            s_to_f: vec![],
        }
    }
}

impl QuizJson {
    fn from_quiz(quiz: Quiz) -> QuizJson {
        QuizJson {
            desc: quiz.desc,
            quiz_data: quiz.quiz_data,
        }
    }
}

impl Quiz {
    pub fn new(title: String, desc: String, original: Vec<Word>) -> Quiz {
        Quiz {
            title,
            desc,
            quiz_data: QuizData::new_from_original(original),
        }
    }

    pub fn new_empty(title: String, desc: String) -> Quiz {
        Quiz {
            title,
            desc,
            quiz_data: QuizData::new_empty(),
        }
    }

    fn from_quiz_json(quiz_json: QuizJson, title: String) -> Quiz {
        Quiz {
            title,
            desc: quiz_json.desc,
            quiz_data: quiz_json.quiz_data,
        }
    }

    pub fn load_data(title: String) -> Quiz {
        // Look later into handling Errors

        Quiz::from_quiz_json(
            serde_json::from_str::<QuizJson>(
                &std::fs::read_to_string(&format!("./quizes/{}.json", title)).unwrap(),
            )
            .unwrap(),
            title,
        )
    }

    pub fn save_data(&self) {
        // Look later into handling Errors
        // I've removed the title from "saving struct", due to difficulties while loading
        std::fs::write(
            format!("./quizes/{}.json", self.title.clone()),
            serde_json::to_string_pretty(&QuizJson::from_quiz(self.clone())).unwrap(),
        )
        .unwrap();
    }
}
