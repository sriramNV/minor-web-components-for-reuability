use eframe::egui::{self, TextEdit, Ui};
use std::fs;

use crate::file_buffer::FileBuffer;
use crate::notepad_app::NotepadApp;

impl NotepadApp {
    pub fn show_tabs(&mut self, ui: &mut Ui) {
        let mut close_tab: Option<usize> = None;

        ui.horizontal_wrapped(|ui| {
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

                        if ui.button("\u{00D7}").clicked() {
                            close_tab = Some(i);
                        }
                    });
                });
            }
        });

        if let Some(i) = close_tab {
            self.buffers.remove(i);
            if self.current_tab >= self.buffers.len() {
                self.current_tab = self.buffers.len().saturating_sub(1);
            }
            if self.editing_tab_index == Some(i) {
                self.editing_tab_index = None;
            }
        }
    }

    pub fn show_editor(&mut self, ui: &mut Ui) {
        if let Some(buffer) = self.buffers.get_mut(self.current_tab) {
            let mut edit = TextEdit::multiline(&mut buffer.content)
                .font(egui::TextStyle::Monospace)
                .frame(true)
                .lock_focus(true);

            if !self.wrap_text {
                edit = edit.desired_width(f32::INFINITY);
            }

            ui.add_sized(ui.available_size(), edit);
            buffer.push_undo();
        }
    }
}