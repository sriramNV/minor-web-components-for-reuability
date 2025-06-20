use eframe::{egui, App, NativeOptions};
use std::fs;
use std::time::{Duration, Instant};

#[derive(PartialEq)]
enum ThemePreference {
    System,
    Light,
    Dark,
}

struct FileBuffer {
    name: String,
    content: String,
    file_path: Option<String>,
    undo_stack: Vec<String>,
    redo_stack: Vec<String>,
    last_edit_time: Instant,
}

impl FileBuffer {
    fn new(name: &str, content: String, path: Option<String>) -> Self {
        Self {
            name: name.to_string(),
            content,
            file_path: path,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            last_edit_time: Instant::now(),
        }
    }

    fn push_undo(&mut self) {
        if self.undo_stack.last().map_or(true, |last| last != &self.content) {
            self.undo_stack.push(self.content.clone());
        }
    }

    fn undo(&mut self) {
        if let Some(prev) = self.undo_stack.pop() {
            self.redo_stack.push(self.content.clone());
            self.content = prev;
        }
    }

    fn redo(&mut self) {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(self.content.clone());
            self.content = next;
        }
    }
}

struct NotepadApp {
    buffers: Vec<FileBuffer>,
    current_tab: usize,
    font_scale: f32,
    wrap_text: bool,
    status: String,
    theme_pref: ThemePreference,
    recent_files: Vec<String>,
    find_query: String,
    replace_query: String,
    show_find: bool,
}

impl Default for NotepadApp {
    fn default() -> Self {
        Self {
            buffers: vec![FileBuffer::new("Untitled", String::new(), None)],
            current_tab: 0,
            font_scale: 1.0,
            wrap_text: true,
            status: "Welcome to Rust Notepad!".into(),
            theme_pref: ThemePreference::System,
            recent_files: Vec::new(),
            find_query: String::new(),
            replace_query: String::new(),
            show_find: false,
        }
    }
}

impl App for NotepadApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let visuals = match self.theme_pref {
            ThemePreference::System => egui::Visuals::default(),
            ThemePreference::Light => egui::Visuals::light(),
            ThemePreference::Dark => egui::Visuals::dark(),
        };
        ctx.set_visuals(visuals);
        ctx.set_pixels_per_point(self.font_scale);

        // Keyboard shortcuts
        let mut do_undo = false;
        let mut do_redo = false;

        ctx.input(|i| {
            if i.key_pressed(egui::Key::Z) && i.modifiers.ctrl {
                do_undo = true;
            }
            if i.key_pressed(egui::Key::Y) && i.modifiers.ctrl {
                do_redo = true;
            }
            if i.key_pressed(egui::Key::F) && i.modifiers.ctrl {
                self.show_find = true;
            }
        });

        // Menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        ui.close_menu();
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            match fs::read_to_string(&path) {
                                Ok(contents) => {
                                    let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                                    self.buffers.push(FileBuffer::new(&name, contents, Some(path.display().to_string())));
                                    self.current_tab = self.buffers.len() - 1;
                                    self.status = format!("Opened: {}", path.display());
                                }
                                Err(err) => self.status = format!("Failed to open: {}", err),
                            }
                        }
                    }

                    if ui.button("Save").clicked() {
                        ui.close_menu();
                        let save_path = self.buffers[self.current_tab].file_path.clone()
                            .or_else(|| rfd::FileDialog::new().save_file().map(|p| p.display().to_string()));
                        if let Some(path) = save_path {
                            match fs::write(&path, &self.buffers[self.current_tab].content) {
                                Ok(_) => {
                                    self.status = format!("Saved: {}", path);
                                    self.buffers[self.current_tab].file_path = Some(path);
                                }
                                Err(err) => self.status = format!("Failed to save: {}", err),
                            }
                        }
                    }

                    if ui.button("Exit").clicked() {
                        std::process::exit(0);
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

        if self.show_find {
            egui::TopBottomPanel::top("find_bar").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Find:");
                    ui.text_edit_singleline(&mut self.find_query);
                    ui.label("Replace:");
                    ui.text_edit_singleline(&mut self.replace_query);
                    if ui.button("Replace All").clicked() {
                        let buffer = &mut self.buffers[self.current_tab];
                        buffer.content = buffer.content.replace(&self.find_query, &self.replace_query);
                    }
                    if ui.button("Close").clicked() {
                        self.show_find = false;
                    }
                });
            });
        }

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.label(&self.status);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                for (i, buf) in self.buffers.iter().enumerate() {
                    let tab = egui::SelectableLabel::new(i == self.current_tab, &buf.name);
                    if ui.add(tab).clicked() {
                        self.current_tab = i;
                    }
                }
            });

            let full_size = ui.max_rect();
            ui.set_min_size(full_size.size());

            egui::ScrollArea::both().auto_shrink([false; 2]).show_viewport(ui, |ui, _| {
                ui.horizontal(|ui| {
                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(40.0, full_size.height()),
                        egui::Layout::top_down(egui::Align::LEFT),
                        |ui| {
                            let buffer = &self.buffers[self.current_tab];
                            let line_count = buffer.content.lines().count().max(1);
                            for i in 1..=line_count {
                                ui.label(format!("{:>4}", i));
                            }
                        },
                    );

                    let buffer = &mut self.buffers[self.current_tab];
                    let mut edit = egui::TextEdit::multiline(&mut buffer.content)
                        .font(egui::TextStyle::Monospace)
                        .frame(true)
                        .lock_focus(true);

                    if !self.wrap_text {
                        edit = edit.desired_width(f32::INFINITY);
                    }

                    ui.add_sized(full_size.size(), edit);
                    buffer.push_undo();
                });
            });
        });

        // After all UI: Perform undo/redo/autosave
        if do_undo {
            self.buffers[self.current_tab].undo();
        }
        if do_redo {
            self.buffers[self.current_tab].redo();
        }

        if self.buffers[self.current_tab].last_edit_time.elapsed() > Duration::from_secs(10) {
            if let Some(path) = &self.buffers[self.current_tab].file_path {
                let _ = fs::write(path, &self.buffers[self.current_tab].content);
                self.status = format!("Autosaved: {}", path);
            }
            self.buffers[self.current_tab].last_edit_time = Instant::now();
        }
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