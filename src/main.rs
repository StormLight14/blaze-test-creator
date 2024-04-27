use serde::{Deserialize, Serialize};
use eframe::egui;

#[derive(Default)]
struct ExamCreatorApp {
    exam_name: String,
    exam_questions: Vec<Question>,
}

impl eframe::App for ExamCreatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
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
                            question.choices.push(format!("Choice {}", question.choices.len() + 1));
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
                }
                if ui.button("Add Question").clicked() {
                    self.exam_questions.push(Question::default());
                }
            });
        });
    }
}

#[derive(Default)]
struct Question {
    prompt: String,
    choices: Vec<String>,
    answer: String,
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
