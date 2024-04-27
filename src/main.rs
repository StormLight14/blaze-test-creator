use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use eframe::egui;

#[derive(Default)]
struct ExamCreatorApp {
    exam_name: String,
    exam_questions: Vec<Question>,
    error_message: Option<String>,
}

impl eframe::App for ExamCreatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
                if let Some(ref error_message) = self.error_message {
                    ui.label(format!("ERROR: {}", error_message));
                }
                ui.horizontal(|ui| {
                    let exam_name_label = ui.label("Exam Name: ");
                    ui.text_edit_singleline(&mut self.exam_name).labelled_by(exam_name_label.id);
                });
                for (i, question) in self.exam_questions.iter_mut().enumerate() {
                    ui.heading(format!("Question {}", i + 1));
                    ui.horizontal(|ui| {
                        let question_prompt_label = ui.label("Question Prompt: ");
                        ui.add(egui::TextEdit::singleline(&mut question.prompt).clip_text(false)).labelled_by(question_prompt_label.id);
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Add Choice").clicked() && question.choices.len() < 8 {
                            question.choices.push(format!("Choice {:?}", Choice::from(question.choices.len())));
                        }
                        for choice in question.choices.iter_mut() {
                            ui.add_sized([80.0, 20.0], egui::TextEdit::singleline(choice).clip_text(false));
                        }
                        if question.choices.len() > 0 {
                            if ui.button("Remove Choice").clicked() {
                                question.choices.pop();
                            }
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Correct Answer: ");
                        for (i, choice) in question.choices.iter().enumerate() {
                            ui.selectable_value(&mut question.answer, Choice::from(i), format!("{:?}", Choice::from(i)));
                        }
                    });
                }
                if ui.button("Add Question").clicked() {
                    self.exam_questions.push(Question::default());
                }
                if ui.button("Save Exam").clicked() {
                    let exam_file = OpenOptions::new().write(true).truncate(true).create(true).open("exam.json");
                    let exam_json = serde_json::to_string::<Exam>(&Exam {
                        name: self.exam_name.clone(),
                        questions: self.exam_questions.clone()
                    });
                    match exam_file {
                        Ok(mut file) => {
                            match exam_json {
                                Ok(exam_json) => {
                                    match file.write_all(exam_json.as_bytes()) {
                                        Ok(_) => (),
                                        Err(err) => self.error_message = Some(err.to_string())
                                    }
                                },
                                Err(err) => {
                                    self.error_message = Some(err.to_string());
                                }
                            }
                        },
                        Err(err) => self.error_message = Some(err.to_string())
                    }
                }
            });
        });
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Exam {
    name: String,
    questions: Vec<Question>
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct Question {
    prompt: String,
    choices: Vec<String>,
    answer: Choice,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
enum Choice {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H
}

impl Default for Choice {
    fn default() -> Self {
        Choice::A
    }
}

impl From<usize> for Choice {
    fn from(num: usize) -> Self {
        match num {
            0 => Choice::A,
            1 => Choice::B,
            2 => Choice::C,
            3 => Choice::D,
            4 => Choice::E,
            5 => Choice::F,
            6 => Choice::G,
            7 => Choice::H,
            _ => Choice::A,
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 360.0]),
        ..Default::default()
    };
    eframe::run_native("BlazeExam Creator", options, Box::new(|_cc| {
        Box::<ExamCreatorApp>::default()
    }))
}
