use crate::notepad_app::NotepadApp;
use crate::notepad_app::ThemePreference;
use eframe::egui;
use std::fs;
use egui::{Context, TextEdit, Key};
use crate::file_buffer::FileBuffer;



pub fn show_menu_bar(app: &mut NotepadApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open").clicked() {
                    ui.close_menu();
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        if let Ok(contents) = fs::read_to_string(&path) {
                            let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                            app.buffers.push(crate::file_buffer::FileBuffer::new(&name, contents, Some(path.display().to_string())));
                            app.current_tab = app.buffers.len() - 1;
                            app.status = format!("Opened: {}", path.display());
                        }
                    }
                }
                if ui.button("Save").clicked() {
                    if let Some(buffer) = app.buffers.get_mut(app.current_tab) {
                        let save_path = buffer.file_path.clone()
                            .or_else(|| rfd::FileDialog::new().save_file().map(|p| p.display().to_string()));
                        if let Some(path) = save_path {
                            if fs::write(&path, &buffer.content).is_ok() {
                                buffer.file_path = Some(path.clone());
                                app.status = format!("Saved: {}", path);
                            }
                        }
                    }
                    ui.close_menu();
                }
                if ui.button("New Tab").clicked() {
                    app.buffers.push(crate::file_buffer::FileBuffer::new("Untitled", String::new(), None));
                    app.current_tab = app.buffers.len() - 1;
                    ui.close_menu();
                }
                if ui.button("Exit").clicked() {
                    std::process::exit(0);
                }
            });

            ui.menu_button("Settings", |ui| {
                ui.label("Font Scale");
                ui.add(egui::Slider::new(&mut app.font_scale, 0.5..=2.5).text("x"));
                ui.checkbox(&mut app.wrap_text, "Wrap lines");
                ui.separator();
                ui.label("Theme");
                ui.radio_value(&mut app.theme_pref, ThemePreference::System, "System");
                ui.radio_value(&mut app.theme_pref, ThemePreference::Light, "Light");
                ui.radio_value(&mut app.theme_pref, ThemePreference::Dark, "Dark");
            });
        });
    });
}

pub fn show_status_bar(app: &NotepadApp, ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
        ui.label(&app.status);
    });
}

pub fn show_find_replace(app: &mut NotepadApp, ctx: &egui::Context) {
    if app.show_find {
        egui::TopBottomPanel::top("find_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Find:");
                ui.text_edit_singleline(&mut app.find_query);
                ui.label("Replace:");
                ui.text_edit_singleline(&mut app.replace_query);
                if ui.button("Replace All").clicked() {
                    if let Some(buffer) = app.buffers.get_mut(app.current_tab) {
                        buffer.content = buffer.content.replace(&app.find_query, &app.replace_query);
                    }
                }
                if ui.button("Close").clicked() {
                    app.show_find = false;
                }
            });
        });
    }
}




pub fn show_tabs_and_editor(ctx: &Context, app: &mut NotepadApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        // Draw tab headers
        ui.horizontal(|ui| {
            for (index, buffer) in app.buffers.iter().enumerate() {
                let tab_title = buffer.name.as_str();

                if ui.selectable_label(app.current_tab == index, tab_title).clicked() {
                    app.current_tab = index;
                }
            }
        });

        ui.separator();

        // Draw the text editor for the selected tab
        if let Some(buffer) = app.buffers.get_mut(app.current_tab) {
            ui.add_sized(
                ui.available_size(), // fills remaining space
                TextEdit::multiline(&mut buffer.content)
                    .font(egui::TextStyle::Monospace)
                    .desired_rows(20)
                    .lock_focus(true),
            );
        }
    });
}

pub fn handle_keyboard_shortcuts(ctx: &Context, app: &mut NotepadApp) {
    let input = ctx.input(|i| i.clone());

    if input.modifiers.ctrl && input.key_pressed(Key::S) {
        if let Some(current) = app.buffers.get_mut(app.current_tab) {
            let _ = current.save(); // Save current tab
        }
    }

    if input.modifiers.ctrl && input.key_pressed(Key::Z) {
        if let Some(current) = app.buffers.get_mut(app.current_tab) {
            current.undo();
        }
    }

    if input.modifiers.ctrl && input.key_pressed(Key::Y) {
        if let Some(current) = app.buffers.get_mut(app.current_tab) {
            current.redo();
        }
    }

    if input.modifiers.ctrl && input.key_pressed(Key::F) {
        app.find_active = true;
    }

    if input.key_pressed(Key::Escape) {
        app.find_active = false;
    }
}

// pub fn show_find_replace(app: &mut NotepadApp, ctx: &egui::Context) {
//     if !app.find_active {
//         return;
//     }

//     egui::TopBottomPanel::top("find_replace_bar").show(ctx, |ui| {
//         ui.horizontal(|ui| {
//             ui.label("Find:");
//             ui.text_edit_singleline(&mut app.find_text);

//             if ui.button("Find Next").clicked() {
//                 if let Some(buffer) = app.buffers.get_mut(app.current_tab) {
//                     if let Some(pos) = buffer.content[app.find_cursor..]
//                         .find(&app.find_text)
//                         .map(|idx| idx + app.find_cursor)
//                     {
//                         app.find_cursor = pos;
//                         // Scroll to cursor or highlight logic can go here
//                     } else {
//                         app.find_cursor = 0; // wrap around
//                     }
//                 }
//             }

//             if ui.button("Close").clicked() {
//                 app.find_active = false;
//             }
//         });
//     });
// }