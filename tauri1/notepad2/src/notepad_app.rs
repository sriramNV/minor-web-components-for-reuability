use crate::file_buffer::FileBuffer;
use crate::ui;
use eframe::egui::{self, Context};
use eframe::App;


#[derive(PartialEq)]
pub enum ThemePreference {
    System,
    Light,
    Dark,
}

pub struct NotepadApp {
    pub buffers: Vec<FileBuffer>,
    pub current_tab: usize,
    pub font_scale: f32,
    pub wrap_text: bool,
    pub status: String,
    pub theme_pref: ThemePreference,
    pub find_query: String,
    pub replace_query: String,
    pub show_find: bool,
    pub editing_tab_index: Option<usize>,
    pub rename_buffer: String,
    pub find_active: bool,
    pub find_text: String,
    pub find_cursor: usize,
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
            find_active: false,
            find_text: String::new(),
            find_cursor: 0,
        }
    }
}

impl App for NotepadApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let visuals = match self.theme_pref {
            ThemePreference::System => egui::Visuals::default(),
            ThemePreference::Light => egui::Visuals::light(),
            ThemePreference::Dark => egui::Visuals::dark(),
        };
        ctx.set_visuals(visuals);
        ctx.set_pixels_per_point(self.font_scale);

        ui::handle_keyboard_shortcuts(ctx, self);
        // ui::autosave_current_tab(self);
        ui::show_menu_bar(self, ctx);
        ui::show_find_replace(self, ctx);
        ui::show_status_bar(self, ctx);
        ui::show_tabs_and_editor(ctx, self);
        // ui::draw_tabs_and_editor(ctx, self);
    }
}