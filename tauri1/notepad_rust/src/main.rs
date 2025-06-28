// A complete Rust Notepad with:
// - Tabs (add/close/rename)
// - Undo/Redo
// - Find/Replace
// - Auto-save
// - Keyboard shortcuts
// Requires: eframe, egui, rfd

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
    find_query: String,
    replace_query: String,
    show_find: bool,
    editing_tab_index: Option<usize>,
    rename_buffer: String,
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
            find_query: String::new(),
            replace_query: String::new(),
            show_find: false,
            editing_tab_index: None,
            rename_buffer: String::new(),
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
            if i.key_pressed(egui::Key::T) && i.modifiers.ctrl {
                self.buffers.push(FileBuffer::new("Untitled", String::new(), None));
                self.current_tab = self.buffers.len() - 1;
            }
        });

        if let Some(buffer) = self.buffers.get_mut(self.current_tab) {
            if buffer.last_edit_time.elapsed() > Duration::from_secs(10) {
                if let Some(path) = &buffer.file_path {
                    let _ = fs::write(path, &buffer.content);
                    self.status = format!("Autosaved: {}", path);
                }
                buffer.last_edit_time = Instant::now();
            }
        }

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        ui.close_menu();
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            if let Ok(contents) = fs::read_to_string(&path) {
                                let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                                self.buffers.push(FileBuffer::new(&name, contents, Some(path.display().to_string())));
                                self.current_tab = self.buffers.len() - 1;
                                self.status = format!("Opened: {}", path.display());
                            }
                        }
                    }
                    if ui.button("Save").clicked() {
                        if let Some(buffer) = self.buffers.get_mut(self.current_tab) {
                            let save_path = buffer.file_path.clone()
                                .or_else(|| rfd::FileDialog::new().save_file().map(|p| p.display().to_string()));
                            if let Some(path) = save_path {
                                if fs::write(&path, &buffer.content).is_ok() {
                                    buffer.file_path = Some(path.clone());
                                    self.status = format!("Saved: {}", path);
                                }
                            }
                        }
                        ui.close_menu();
                    }
                    if ui.button("New Tab").clicked() {
                        self.buffers.push(FileBuffer::new("Untitled", String::new(), None));
                        self.current_tab = self.buffers.len() - 1;
                        ui.close_menu();
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
                        if let Some(buffer) = self.buffers.get_mut(self.current_tab) {
                            buffer.content = buffer.content.replace(&self.find_query, &self.replace_query);
                        }
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
            ui.horizontal_wrapped(|ui| {
                let mut close_tab: Option<usize> = None;

                for i in 0..self.buffers.len() {
                    let is_selected = i == self.current_tab;
                    let is_renaming = self.editing_tab_index == Some(i);

                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            if is_renaming {
                                let response = ui.text_edit_singleline(&mut self.rename_buffer);
                                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                    if let Some(tab) = self.buffers.get_mut(i) {
                                        tab.name = self.rename_buffer.clone();
                                        if let Some(path) = &tab.file_path {
                                            let _ = fs::write(path, &tab.content);
                                            self.status = format!("Renamed and saved: {}", path);
                                        }
                                    }
                                    self.editing_tab_index = None;
                                }
                            } else {
                                let tab_name = &self.buffers[i].name;
                                let label = if is_selected {
                                    format!("[{}]", tab_name)
                                } else {
                                    tab_name.clone()
                                };

                                let response = ui.selectable_label(is_selected, label);
                                if response.clicked() {
                                    self.current_tab = i;
                                }
                                if response.double_clicked() {
                                    self.rename_buffer = tab_name.clone();
                                    self.editing_tab_index = Some(i);
                                }
                            }

                            if ui.button("×").clicked() {
                                close_tab = Some(i);
                            }
                        });
                    });
                }

                if let Some(i) = close_tab {
                    self.buffers.remove(i);
                    if self.current_tab >= self.buffers.len() {
                        self.current_tab = self.buffers.len().saturating_sub(1);
                    }
                    if self.editing_tab_index == Some(i) {
                        self.editing_tab_index = None;
                    }
                }
            });

            egui::ScrollArea::both().auto_shrink([false; 2]).show(ui, |ui| {
                if let Some(buffer) = self.buffers.get_mut(self.current_tab) {
                    let mut edit = egui::TextEdit::multiline(&mut buffer.content)
                        .font(egui::TextStyle::Monospace)
                        .frame(true)
                        .lock_focus(true);

                    if !self.wrap_text {
                        edit = edit.desired_width(f32::INFINITY);
                    }

                    ui.add_sized(ui.available_size(), edit);
                    buffer.push_undo();
                }
            });
        });

        if do_undo {
            if let Some(buffer) = self.buffers.get_mut(self.current_tab) {
                buffer.undo();
            }
        }
        if do_redo {
            if let Some(buffer) = self.buffers.get_mut(self.current_tab) {
                buffer.redo();
            }
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