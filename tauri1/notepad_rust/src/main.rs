use eframe::{egui, App, NativeOptions};
use std::fs;

#[derive(PartialEq)]
enum ThemePreference {
    System,
    Light,
    Dark,
}

struct NotepadApp {
    text: String,
    file_path: Option<String>,
    recent_files: Vec<String>,
    font_scale: f32,
    wrap_text: bool,
    status: String,
    theme_pref: ThemePreference,
}

impl Default for NotepadApp {
    fn default() -> Self {
        Self {
            text: String::new(),
            file_path: None,
            recent_files: Vec::new(),
            font_scale: 1.0,
            wrap_text: true,
            status: "Welcome to Rust Notepad!".into(),
            theme_pref: ThemePreference::System,
        }
    }
}

impl App for NotepadApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Theme setting
        let visuals = match self.theme_pref {
            ThemePreference::System => egui::Visuals::default(),
            ThemePreference::Light => egui::Visuals::light(),
            ThemePreference::Dark => egui::Visuals::dark(),
        };
        ctx.set_visuals(visuals);
        ctx.set_pixels_per_point(self.font_scale);

        // Menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        ui.close_menu();
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            match fs::read_to_string(&path) {
                                Ok(contents) => {
                                    self.text = contents;
                                    self.file_path = Some(path.display().to_string());
                                    self.status = format!("Opened: {}", path.display());
                                    if !self.recent_files.contains(&path.display().to_string()) {
                                        self.recent_files.insert(0, path.display().to_string());
                                        if self.recent_files.len() > 5 {
                                            self.recent_files.pop();
                                        }
                                    }
                                }
                                Err(err) => {
                                    self.status = format!("Failed to open: {}", err);
                                }
                            }
                        }
                    }
                    if ui.button("Save").clicked() {
                        ui.close_menu();
                        let save_path = self.file_path.clone()
                            .or_else(|| rfd::FileDialog::new().save_file().map(|p| p.display().to_string()));
                        if let Some(path) = save_path {
                            match fs::write(&path, &self.text) {
                                Ok(_) => {
                                    self.status = format!("Saved: {}", path);
                                    self.file_path = Some(path.clone());
                                    if !self.recent_files.contains(&path) {
                                        self.recent_files.insert(0, path);
                                        if self.recent_files.len() > 5 {
                                            self.recent_files.pop();
                                        }
                                    }
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

                ui.menu_button("Recent", |ui| {
                    for path in &self.recent_files {
                        if ui.button(path).clicked() {
                            match fs::read_to_string(path) {
                                Ok(contents) => {
                                    self.text = contents;
                                    self.file_path = Some(path.clone());
                                    self.status = format!("Opened: {}", path);
                                }
                                Err(err) => {
                                    self.status = format!("Failed to open: {}", err);
                                }
                            }
                        }
                    }
                });

                ui.menu_button("Settings", |ui| {
                    ui.label("Font Scale");
                    ui.add(egui::Slider::new(&mut self.font_scale, 0.5..=2.5).text("x"));
                    ui.checkbox(&mut self.wrap_text, "Wrap lines");
                    ui.separator();
                    ui.label("Theme");
                    ui.radio_value(&mut self.theme_pref, ThemePreference::System, "System");
                    ui.radio_value(&mut self.theme_pref, ThemePreference::Light, "Light");
                    ui.radio_value(&mut self.theme_pref, ThemePreference::Dark, "Dark");
                });
            });
        });

        // Status bar
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.label(&self.status);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
    let full_size = ui.max_rect(); // Get full window space
    ui.set_min_size(full_size.size());

    egui::ScrollArea::both().auto_shrink([false; 2]).show_viewport(ui, |ui, _| {
        ui.horizontal(|ui| {
            // Line numbers
            ui.allocate_ui_with_layout(
                egui::Vec2::new(40.0, full_size.height()),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    let line_count = self.text.lines().count().max(1);
                    for i in 1..=line_count {
                        ui.label(format!("{:>4}", i));
                    }
                },
            );

            // Text editor
            let mut edit = egui::TextEdit::multiline(&mut self.text)
                .font(egui::TextStyle::Monospace)
                .frame(true)
                .lock_focus(true);

            if !self.wrap_text {
                edit = edit.desired_width(f32::INFINITY);
            }

            ui.add_sized(full_size.size(), edit);
            // ui.painter().rect_filled(full_size, 0.0, egui::Color32::RED);

        });
    });
});

    }
}

fn main() -> Result<(), eframe::Error> {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "Rust Notepad",
        native_options,
        Box::new(|_cc| Ok(Box::new(NotepadApp::default()))),
    )
}
