//! Theme preference handling for the Rust Notepad app.

use eframe::egui;

#[derive(PartialEq)]
pub enum ThemePreference {
    System,
    Light,
    Dark,
}

impl ThemePreference {
    pub fn apply(&self, ctx: &egui::Context) {
        let visuals = match self {
            ThemePreference::System => egui::Visuals::default(),
            ThemePreference::Light => egui::Visuals::light(),
            ThemePreference::Dark => egui::Visuals::dark(),
        };
        ctx.set_visuals(visuals);
    }
}