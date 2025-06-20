use eframe::{egui, App};
use std::fs;

struct NotepadApp {
    text: String,
    file_path: Option<String>,
    status: String,
}

impl Default for NotepadApp {
    fn default() -> Self {
        Self {
            text: String::new(),
            file_path: None,
            status: "Welcome to Rust Notepad!".into(),
        }
    }
}

impl App for NotepadApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Open").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        match fs::read_to_string(&path) {
                            Ok(contents) => {
                                self.text = contents;
                                self.file_path = Some(path.display().to_string());
                                self.status = format!("Opened: {}", path.display());
                            }
                            Err(err) => {
                                self.status = format!("Failed to open: {}", err);
                            }
                        }
                    }
                }

                if ui.button("Save").clicked() {
                    if let Some(path) = rfd::FileDialog::new().save_file() {
                        match fs::write(&path, &self.text) {
                            Ok(_) => {
                                self.status = format!("Saved to: {}", path.display());
                            }
                            Err(err) => {
                                self.status = format!("Failed to save: {}", err);
                            }
                        }
                    }
                }

                if ui.button("Exit").clicked() {
                    std::process::exit(0);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(&self.status);
            ui.separator();
            ui.add(
                egui::TextEdit::multiline(&mut self.text)
                    .hint_text("Start typing...")
                    .desired_rows(30),
            );
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust Notepad",
        options,
        Box::new(|_cc| Ok(Box::new(NotepadApp::default()))),
    )
}

